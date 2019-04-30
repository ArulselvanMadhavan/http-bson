extern crate actix;
extern crate actix_web;
extern crate serde;
extern crate serde_json;
use actix_web::*;

fn index(info: Path<(String, u32)>) -> String {
    format!("Hello {}! id:{}", info.0, info.1)
}

fn main() {
    server::new(|| App::new().resource("/{collectionName}/{id}", |r| r.with(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
