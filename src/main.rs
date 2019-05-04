extern crate actix;
extern crate actix_web;
extern crate bson;
extern crate im;
extern crate serde;
extern crate serde_json;
use actix_web::error::Result;
use actix_web::*;
use bson::oid::ObjectId;
use bson::Document;
use im::hashmap::HashMap;
use std::fs::File;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;

#[derive(Fail, Debug)]
enum AppError {
    #[fail(display = "Invalid Object Id")]
    InvalidObjectId,
    #[fail(display = "ObjectId Not Found")]
    ObjectIdNotFound,
    #[fail(display = "Document doesn't have expected structure")]
    CorruptDocument,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::InvalidObjectId => HttpResponse::new(http::StatusCode::BAD_REQUEST),
            AppError::ObjectIdNotFound => HttpResponse::new(http::StatusCode::NOT_FOUND),
            AppError::CorruptDocument => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
fn get_base_template(
    hs: &'static HashMap<ObjectId, Document>,
    oid: &str,
) -> Result<BaseTemplate, AppError> {
    let oid = ObjectId::with_string(oid).map_err(|_e| AppError::InvalidObjectId)?;
    let doc = hs.get(&oid).ok_or(AppError::ObjectIdNotFound)?;
    let bt = match (doc.get_object_id("_id"), doc.get_str("name")) {
        (Ok(oid), Ok(name)) => Ok(BaseTemplate {
            id: oid.to_string(),
            name: name.to_string(),
        }),
        (_, _) => Err(AppError::CorruptDocument),
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

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse> {
        let body = serde_json::to_string(&self)?;
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn find_and_return(
    hs: &'static HashMap<ObjectId, Document>,
) -> impl Fn(Path<(String, String)>) -> Result<BaseTemplate, AppError> {
    move |info| get_base_template(hs, info.1.as_str())
}

// Sample: 57d0c3f3f6cd4530aa50ea18
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

    server::new(|| {
        App::new().resource("/{collectionName}/{id}", |r| {
            r.with(find_and_return(&HASHMAP))
        })
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}
