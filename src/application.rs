use context::{BasicContext, Context};
use hyper::{Body, Request, Server};
use middleware::{Middleware, MiddlewareChain};
use radix_router::router::{Params, Router};

pub struct Application<T: 'static + Send> {
    pub middleware: Vec<Middleware<T>>,
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

    pub fn listen(&self) {}
}
