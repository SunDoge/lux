use hyper::{Body, Request};
use radix_router::router::Params;

pub trait Context {
    fn index(&mut self) -> usize;
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
}