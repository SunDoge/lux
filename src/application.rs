use context::{BasicContext, Context};
use futures::future;
use hyper::rt::{self, Future, Stream};
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use middleware::{Middleware, MiddlewareChain};
use radix_router::path::clean_path;
use radix_router::router::{BoxFut, Params, Router};
use std::sync::Arc;

impl<T: Context> Middleware<T> for Router<Vec<Box<Middleware<T> + Send + Sync>>> {
    fn call(&self, mut context: T, m: &MiddlewareChain<T>) -> BoxFut {
        let root = self.trees.get(context.request().method().as_str());
        if let Some(root) = root {
            let (handle, ps, tsr) = root.get_value(context.request().uri().path());

            if let Some(handle) = handle {
                // return handle(req, response, ps);
                // return handle.handle(req, ps);
                let m = MiddlewareChain {
                    // middleware: cloned_app.stack.clone(),
                    middleware: handle, // middleware: &ref_app.middleware
                };
                context.reset();
                *context.params() = ps;
                return m.next(context);
            } else if context.request().method() != &Method::CONNECT
                && context.request().uri().path() != "/"
            {
                let code = if context.request().method() != &Method::GET {
                    // StatusCode::from_u16(307).unwrap()
                    307
                } else {
                    // StatusCode::from_u16(301).unwrap()
                    301
                };

                // if tsr && self.redirect_trailing_slash {
                if tsr && true {
                    let path = if context.request().uri().path().len() > 1
                        && context.request().uri().path().ends_with("/")
                    {
                        context.request().uri().path()[..context.request().uri().path().len() - 1]
                            .to_string()
                    } else {
                        context.request().uri().path().to_string() + "/"
                    };

                    // response.headers_mut().insert(header::LOCATION, header::HeaderValue::from_str(&path).unwrap());
                    // *response.status_mut() = code;
                    let response = Response::builder()
                        .header("Location", path.as_str())
                        .status(code)
                        .body(Body::empty())
                        .unwrap();
                    return Box::new(future::ok(response));
                }

                // if self.redirect_fixed_path {
                if true {
                    let (fixed_path, found) = root.find_case_insensitive_path(
                        &clean_path(context.request().uri().path()),
                        // self.redirect_trailing_slash,
                        true,
                    );

                    if found {
                        //  response.headers_mut().insert(header::LOCATION, header::HeaderValue::from_str(&fixed_path).unwrap());
                        // *response.status_mut() = code;
                        let response = Response::builder()
                            .header("Location", fixed_path.as_str())
                            .status(code)
                            .body(Body::empty())
                            .unwrap();
                        return Box::new(future::ok(response));
                    }
                }
            }
        }

        // if context.request().method() == &Method::OPTIONS && self.handle_options {
        if context.request().method() == &Method::OPTIONS && true {
            let allow = self.allowed(
                context.request().uri().path(),
                context.request().method().as_str(),
            );
            if allow.len() > 0 {
                // *response.headers_mut().get_mut("allow").unwrap() = header::HeaderValue::from_str(&allow).unwrap();
                let response = Response::builder()
                    .header("Allow", allow.as_str())
                    .body(Body::empty())
                    .unwrap();
                return Box::new(future::ok(response));
            }
        } else {
            // if self.handle_method_not_allowed {
            if true {
                let allow = self.allowed(
                    context.request().uri().path(),
                    context.request().method().as_str(),
                );

                if allow.len() > 0 {
                    let mut response = Response::builder()
                        .header("Allow", allow.as_str())
                        .body(Body::empty())
                        .unwrap();

                    // if let Some(ref method_not_allowed) = self.method_not_allowed {
                    //     return method_not_allowed.handle(req, Params::new());
                    // } else {
                    *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
                    *response.body_mut() = Body::from("METHOD_NOT_ALLOWED");
                    // }

                    return Box::new(future::ok(response));
                }
            }
        }

        // Handle 404
        // if let Some(ref not_found) = self.not_found {
        //     return not_found.handle(req, Params::new());
        // } else {
        // *response.status_mut() = StatusCode::NOT_FOUND;
        let response = Response::builder()
            .status(404)
            .body("NOT_FOUND".into())
            .unwrap();
        return Box::new(future::ok(response));
        // }
    }
}

pub struct Application<T: 'static + Send> {
    pub router: Vec<(
        &'static str,
        &'static str,
        Vec<Box<Middleware<T> + Send + Sync>>,
    )>,
    pub middleware: Vec<Box<Middleware<T> + Send + Sync>>,
    pub context_generator: fn(Request<Body>) -> T,
}

fn generate_context(req: Request<Body>) -> BasicContext {
    BasicContext {
        request: req,
        params: Params::new(),
        index: 0,
    }
}

impl<T: Context + Send> Application<T> {
    pub fn new() -> Application<BasicContext> {
        Application {
            router: Vec::new(),
            middleware: Vec::new(),
            context_generator: generate_context,
        }
    }

    pub fn from(generator: fn(Request<Body>) -> T) -> Application<T> {
        Application {
            router: Vec::new(),
            middleware: Vec::new(),
            context_generator: generator,
        }
    }

    pub fn handle(&mut self, method: &'static str, path: &'static str, m: Vec<Box<Middleware<T> + Send + Sync>>) {
        self.router.push((method, path, m));
    }

    pub fn use_middleware(
        &mut self,
        middleware: impl Middleware<T> + Send + Sync + 'static,
    ) -> &mut Self {
        self.middleware.push(Box::new(middleware));
        self
    }

    pub fn use_router(&mut self) {
        let mut router = Router::<Vec<Box<Middleware<T> + Send + Sync>>>::new();
        while self.router.len() > 0 {
            let route = self.router.pop().unwrap();
            router.handle(route.0, route.1, route.2);
        }
        self.use_middleware(router);
    }

    // pub fn handle(&mut self, method: &str, path: &str, m: Vec<Box<Middleware<T> + Send + Sync>>) {
    //     self.router.handle(method, path, m);
    // }

    pub fn run(mut app: Application<T>, addr: &str) {
        // let addr = ([127, 0, 0, 1], 3000).into();

        let arc_app = Arc::new(app);
        // new_service is run for each connection, creating a 'service'
        // to handle requests for that specific connection.
        let new_service = move || {
            // This is the `Service` that will handle the connection.
            // `service_fn_ok` is a helper to convert a function that
            // returns a Response into a `Service`.
            let cloned_app = arc_app.clone();
            // let ref_app = &app;

            service_fn(move |req| -> BoxFut {
                let context = (cloned_app.context_generator)(req);
                // let context = (ref_app.context_generator)(req);
                let m = MiddlewareChain {
                    // middleware: cloned_app.stack.clone(),
                    middleware: &cloned_app.middleware, // middleware: &ref_app.middleware
                };
                m.next(context)
                // Box::new(future::ok(Response::new(Body::empty())))
            })
        };

        let server = Server::bind(&addr.parse().unwrap())
            .serve(new_service)
            .map_err(|e| eprintln!("server error: {}", e));

        println!("Listening on http://{}", addr);

        rt::run(server);
    }
}
