use context::Context;
use radix_router::router::BoxFut;
use std::sync::Arc;

// pub type Middleware<T> = fn(T, chain: &MiddlewareChain<T>) -> BoxFut;
pub trait Middleware<T> {
    fn call(&self, T, &MiddlewareChain<T>) -> BoxFut;
}

impl<T, F> Middleware<T> for F
where
    F: Fn(T, &MiddlewareChain<T>) -> BoxFut,
{
    fn call(&self, context: T, m: &MiddlewareChain<T>) -> BoxFut {
        (self)(context, m)
    }
}

pub struct MiddlewareChain<'a, T: 'a> {
    // pub middleware: Arc<Vec<Box<Middleware<T> + Send + Sync>>>,
    pub middleware: &'a Vec<Box<Middleware<T> + Send + Sync>>,
}

impl<'a, T: Context> MiddlewareChain<'a, T> {
    pub fn next(&self, mut context: T) -> BoxFut {
        let next_middleware = self.middleware.get(context.index()).unwrap();
        next_middleware.call(context, self)
    }
}


