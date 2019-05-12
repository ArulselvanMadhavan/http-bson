pub mod actor_errors;
pub mod app_errors;
use actix::MailboxError;
use actix_web::error::ResponseError;
use actix_web::HttpResponse;
use actor_errors::ActorError;
use app_errors::AppError;
// use serde_json::{Map as JsonMap, Value as JsonValue};
#[derive(Fail, Debug)]
pub enum Error {
    //400
    #[fail(display = "Bad Request")]
    BadRequest,
    // 404
    #[fail(display = "Not Found")]
    NotFound,
    // 500
    #[fail(display = "Internal Server Error")]
    InternalServerError,
}

impl From<ActorError> for Error {
    fn from(_error: ActorError) -> Self {
        Error::InternalServerError
    }
}

impl From<MailboxError> for Error {
    fn from(_error: MailboxError) -> Self {
        Error::InternalServerError
    }
}

impl From<AppError> for Error {
    fn from(error: AppError) -> Self {
        match error {
            AppError::ObjectIdNotFound => Error::NotFound,
            AppError::InvalidObjectId => Error::BadRequest,
            AppError::CorruptDocument => Error::InternalServerError,
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            Error::NotFound => HttpResponse::NotFound().json("Not Found"),
            Error::BadRequest => HttpResponse::BadRequest().json("Bad Request"),
        }
    }
}
