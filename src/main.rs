extern crate actix;
extern crate actix_web;
extern crate serde;
extern crate serde_json;
use actix_web::*;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello World!"
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
