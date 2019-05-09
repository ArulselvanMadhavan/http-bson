use actix_web::error;
use actix_web::{http, HttpResponse};

#[derive(Fail, Debug)]
pub enum AppError {
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
