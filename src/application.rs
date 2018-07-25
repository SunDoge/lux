use context::{BasicContext, Context};
use futures::future;
use hyper::rt::{self, Future, Stream};
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};
use middleware::{Middleware, MiddlewareChain};
use radix_router::router::{BoxFut, Params, Router};
use std::sync::Arc;

pub struct Application<T: 'static + Send> {
    // pub router: Router<Vec<Box<Middleware<T> + Send + Sync>>>,
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
            middleware: Vec::new(),
            context_generator: generate_context,
        }
    }

    pub fn from(generator: fn(Request<Body>) -> T) -> Application<T> {
        Application {
            middleware: Vec::new(),
            context_generator: generator,
        }
    }

    pub fn use_middleware(&mut self, middleware: impl Middleware<T> + Send + Sync + 'static) -> &mut Self {
        self.middleware.push(Box::new(middleware));
        self
    }

    // pub fn handle(&mut self, method: &str, path: &str, m: Vec<Box<Middleware<T> + Send + Sync>>) {
    //     self.router.handle(method, path, m);
    // }

    pub fn run(app: Application<T>, addr: &str) {
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
                    middleware: &cloned_app.middleware
                    // middleware: &ref_app.middleware
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
