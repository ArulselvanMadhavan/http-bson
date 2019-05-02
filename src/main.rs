extern crate actix;
extern crate actix_web;
extern crate bson;
extern crate im;
extern crate serde;
extern crate serde_json;
use actix_web::*;
use bson::oid::ObjectId;
use bson::Document;
use im::hashmap::HashMap;
use std::fs::File;
use std::result::Result;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

// fn index(info: Path<(String, u32)>) -> String {
//     format!("Hello {}! id:{}", info.0, info.1)
// }
fn index(bt: Option<BaseTemplate>) -> impl Responder {
    bt.expect("Stupid effin actix")
}

fn get_base_template(hs: &HashMap<ObjectId, Document>) -> Option<BaseTemplate> {
    let oid = ObjectId::with_string("57d0c3f3f6cd4530aa50ea18").expect("F U");
    let doc = hs.get(&oid).expect("Effing doc is missing in hash map");
    let bt = match (doc.get_object_id("_id"), doc.get_str("name")) {
        (Ok(oid), Ok(name)) => Some(BaseTemplate {
            id: oid.to_string(),
            name: name.to_string(),
        }),
        (_, _) => None,
    };
    bt
}
#[derive(Serialize, Debug, Clone)]
struct BaseTemplate {
    id: String, // Move them to references
    name: String,
}

impl Responder for BaseTemplate {
    type Item = HttpResponse;
    type Error = actix_web::Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, actix_web::Error> {
        let body = serde_json::to_string(&self)?;
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn main() -> () {
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
    lazy_static! {
        static ref SAMPLE_BASETEMPLATE: Option<BaseTemplate> = get_base_template(&HASHMAP);
    }
    server::new(|| {
        App::new().resource("/{collectionName}/{id}", |r| {
            r.f(|_req| index(SAMPLE_BASETEMPLATE.clone()))
        })
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}
