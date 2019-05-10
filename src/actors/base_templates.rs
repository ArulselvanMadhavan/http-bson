use crate::errors::app_errors::AppError;
use actix::prelude::{Actor, Handler, Message, SyncContext};
use actix_web;
use actix_web::error::Result;
use actix_web::{HttpRequest, HttpResponse, Responder};
use bson::oid::ObjectId;
use bson::Document;
use im::hashmap::HashMap;

struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

pub struct BaseTemplatesActor {
    pub count: usize,
}

impl Actor for BaseTemplatesActor {
    type Context = SyncContext<Self>;
}

impl Handler<Ping> for BaseTemplatesActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _: &mut SyncContext<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct BaseTemplate {
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

pub fn get_base_template(
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
