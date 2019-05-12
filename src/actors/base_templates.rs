use crate::app::AppState;
use crate::errors::actor_errors::*;
use crate::errors::app_errors::AppError;
use crate::errors::Error;
use actix::prelude::{Actor, Handler, Message, SyncContext};
use actix::MailboxError;
use actix_web;
use actix_web::error::{ResponseError, Result};
use actix_web::{HttpRequest, HttpResponse, Path, Responder};
use bson::oid::ObjectId;
use bson::Document;
use futures::future;
use futures::Future;
use im::hashmap::HashMap;
struct Ping;

impl Message for Ping {
    type Result = usize; // Change to boolean
}

struct GetRecord {
    oid: ObjectId,
}

impl Message for GetRecord {
    type Result = Option<BaseTemplate>;
}

pub struct BaseTemplatesActor {
    pub count: usize,
}

impl Actor for BaseTemplatesActor {
    type Context = SyncContext<Self>;
}

impl Handler<Ping> for BaseTemplatesActor {
    type Result = usize;

    fn handle(&mut self, _: Ping, _: &mut SyncContext<Self>) -> Self::Result {
        self.count += 1;
        self.count
    }
}

impl Handler<GetRecord> for BaseTemplatesActor {
    type Result = Option<BaseTemplate>;

    fn handle(&mut self, _msg: GetRecord, _: &mut SyncContext<Self>) -> Self::Result {
        Some(BaseTemplate {
            id: "some_id".to_string(),
            name: "some_name".to_string(),
        })
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
    oid_as_str: &str,
    req: HttpRequest<AppState>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let bt_actor = req.state().bt.clone();
    future::result(ObjectId::with_string(oid_as_str))
        .map_err(|_e| AppError::InvalidObjectId)
        .from_err() // Lift AppError to Error
        .and_then(move |oid| {
            bt_actor.send(GetRecord { oid }).from_err() // Lift MailboxError to Error
        })
        .and_then(|res| {
            future::result(res.ok_or(AppError::ObjectIdNotFound))
                .map(|bt| HttpResponse::Ok().json(bt))
                .from_err()
        })
}

pub fn ping_and_respond(
    (path, req): (Path<String>, HttpRequest<AppState>),
) -> impl Future<Item = HttpResponse, Error = ActorError> {
    let bt_actor = req.state().bt.clone();
    bt_actor
        .send(Ping)
        .map(move |res| BaseTemplate {
            id: res.to_string(),
            name: path.to_string(),
        })
        .map(|bt| HttpResponse::Ok().json(bt))
        .from_err()
}
