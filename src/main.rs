extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate koa_rs;
extern crate radix_router;

use futures::future;
use hyper::rt::Future;
use hyper::{Body, Response};
use koa_rs::application::Application;
use koa_rs::context::BasicContext;
use koa_rs::middleware::MiddlewareChain;
use radix_router::router::BoxFut;
use std::sync::Arc;

fn m1(ctx: BasicContext, m: &MiddlewareChain<BasicContext>) -> BoxFut {
    println!("Before");
    Box::new(m.next(ctx).and_then(|res| {
        println!("After");
        future::ok(res)
    }))
}

fn m2(ctx: BasicContext, m: &MiddlewareChain<BasicContext>) -> BoxFut {
    Box::new(future::ok(Response::new(Body::from("hello"))))
}

type Handler = fn(BasicContext, &MiddlewareChain<BasicContext>) -> BoxFut;

fn main() {
    let mut app = Application::<BasicContext>::new();
    app.stack = Arc::new(vec![Box::new(m1 as Handler), Box::new(m2 as Handler)]);
    Application::listen(app);
}
