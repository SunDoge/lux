use context::Context;
use radix_router::router::BoxFut;

pub type Middleware<T> = fn(T, chain: &MiddlewareChain<T>) -> BoxFut;

pub struct MiddlewareChain<'a, T: 'a> {
    pub middleware: &'a Vec<Middleware<T>>,
}

impl<'a, T: Context> MiddlewareChain<'a, T> {
    pub fn next(&self, mut context: T) -> BoxFut {
        let next_middleware = self.middleware.get(context.index()).unwrap();
        next_middleware(context, self)
    }
}
