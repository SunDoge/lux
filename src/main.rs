extern crate env_logger;
extern crate koa_rs;

use koa_rs::application::Application;
use koa_rs::context::BasicContext;

fn main() {
    let app = Application::<BasicContext>::new();
    app.listen();
}
