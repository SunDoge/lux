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
use radix_router::router::{BoxFut, Param, Params};
use std::sync::Arc;

fn m1(ctx: BasicContext, m: &MiddlewareChain<BasicContext>) -> BoxFut {
    // println!("Before");
    Box::new(m.next(ctx).and_then(|res| {
        // println!("After");
        future::ok(res)
    }))
}

fn m2(ctx: BasicContext, m: &MiddlewareChain<BasicContext>) -> BoxFut {
    Box::new(future::ok(Response::new(Body::from(format!(
        "hello, {}",
        &ctx.params[0]
    )))))
}

fn m3(ctx: BasicContext, m: &MiddlewareChain<BasicContext>) -> BoxFut {
    Box::new(future::ok(Response::new(Body::from("hello"))))
}

type Handler = fn(BasicContext, &MiddlewareChain<BasicContext>) -> BoxFut;

fn main() {
    let mut app = Application::<BasicContext>::new();
    // app.stack = Arc::new(vec![Box::new(m1 as Handler), Box::new(m2 as Handler)]);
    app.use_middleware(m1);
    // app.use_middleware(
    //     |mut ctx: BasicContext, m: &MiddlewareChain<BasicContext>| -> BoxFut {
    //         ctx.params = Params(vec![Param::new("key", "value")]);
    //         m.next(ctx)
    //     },
    // );
    // app.use_middleware(m2);
    app.handle("GET", "/", vec![Box::new(m3)]);
    app.handle("GET", "/to/:key", vec![Box::new(m2)]);
    app.use_router();
    Application::run(app, "127.0.0.1:3000");
}
