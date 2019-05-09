use actix_web::error::Result;
use actix_web::*;
use bson::oid::ObjectId;
use bson::Document;
use im::hashmap::HashMap;
use listenfd::ListenFd;
use std::fs::File;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;

mod actors;
mod errors;
use actors::base_templates::{get_base_template, BaseTemplate};
use errors::app_errors::AppError;

fn find_and_return(
    hs: &'static HashMap<ObjectId, Document>,
) -> impl Fn(Path<(String, String)>) -> Result<BaseTemplate, AppError> {
    move |info| get_base_template(hs, info.1.as_str())
}

// Sample: 57d0c3f3f6cd4530aa50ea18
fn main() -> () {
    let mut listenfd = ListenFd::from_env();
    lazy_static! {
        static ref HASHMAP: HashMap<ObjectId, Document> = {
            let mut f = File::open("samples/base_templates.bson").unwrap();
            let mut hs = HashMap::new();
            while let Ok(decoded) = bson::decode_document(&mut f) {
             if let Ok(id) = decoded.get_object_id("_id") {
              hs.insert(id.clone(), decoded.clone()); // TODO: How to avoid cloning?
             }
           }
            hs
        };
    }

    let mut server = server::new(|| {
        App::new().resource("/{collectionName}/{id}", |r| {
            r.with(find_and_return(&HASHMAP))
        })
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:8088").unwrap()
    };
    server.run();
}
