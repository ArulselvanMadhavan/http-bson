use crate::actors::base_templates::{get_base_template, BaseTemplate, BaseTemplatesActor};
use crate::errors::app_errors::AppError;
use actix::prelude::Addr;
use actix::sync::SyncArbiter;
use actix_web::{App, Path};
use bson::oid::ObjectId;
use bson::Document;
use im::hashmap::HashMap;

fn find_and_return(
    hs: &'static HashMap<ObjectId, Document>,
) -> impl Fn(Path<(String, String)>) -> Result<BaseTemplate, AppError> {
    move |info| get_base_template(hs, info.1.as_str())
}

pub fn create(hs: &'static HashMap<ObjectId, Document>) -> App {
    let bt_address: Addr<BaseTemplatesActor> =
        SyncArbiter::start(1, || BaseTemplatesActor { count: 0 });
    App::new().resource("/{collectionName}/${id}", move |r| {
        r.with(find_and_return(hs))
    })
}
