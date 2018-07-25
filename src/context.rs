use hyper::{Body, Request};
use middleware::MiddlewareChain;
use radix_router::router::{BoxFut, Params};

pub trait Context {
    fn index(&mut self) -> usize;
    fn request(&self) -> &Request<Body>;
    fn params(&mut self) -> &mut Params;
    fn reset(&mut self);
}

pub struct BasicContext {
    pub request: Request<Body>,
    pub params: Params,
    pub index: usize,
}

impl Context for BasicContext {
    fn index(&mut self) -> usize {
        self.index += 1;
        self.index - 1
    }

    fn request(&self) -> &Request<Body> {
        &self.request
    }

    fn params(&mut self) -> &mut Params {
        &mut self.params
    }

    fn reset(&mut self) {
        self.index = 0;
    }
}
