use actix::MailboxError;
use actix_web::error;
use actix_web::{http, HttpResponse};

#[derive(Fail, Debug)]
pub enum ActorError {
    #[fail(display = "Internal Server Error")]
    MailboxClosed,
    #[fail(display = "Internal Server Error")]
    MailboxTimeout,
}

impl From<MailboxError> for ActorError {
    fn from(error: MailboxError) -> ActorError {
        match error {
            MailboxError::Closed => ActorError::MailboxClosed,
            MailboxError::Timeout => ActorError::MailboxTimeout,
        }
    }
}

impl error::ResponseError for ActorError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ActorError::MailboxClosed => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
            ActorError::MailboxTimeout => {
                HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
