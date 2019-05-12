use crate::actors::base_templates::{
    get_base_template, ping_and_respond, BaseTemplate, BaseTemplatesActor,
};
use crate::errors::app_errors::AppError;
use actix::prelude::Addr;
use actix::sync::SyncArbiter;
use actix_web::{App, HttpRequest, Path};
use bson::oid::ObjectId;
use bson::Document;
use im::hashmap::HashMap;

pub struct AppState {
    pub bt: Addr<BaseTemplatesActor>,
}

pub fn create(hs: &'static HashMap<ObjectId, Document>) -> App<AppState> {
    let bt_address: Addr<BaseTemplatesActor> =
        SyncArbiter::start(1, || BaseTemplatesActor { count: 0 });
    let state = AppState {
        bt: bt_address.clone(),
    };
    App::with_state(state)
        .resource("/{collectionName}/ping", move |r| {
            r.with_async(ping_and_respond)
        })
        .resource("/base_templates/{id}", move |r| {
            r.with_async(move |(info, req): (Path<String>, HttpRequest<AppState>)| {
                get_base_template(hs, info.as_str(), req)
            })
        })
}
