use actix_web::{dev::HttpResponseBuilder, error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Display, Error, Serialize)]
pub enum ErrorCodes {
    #[display(fmt = "Invalid ID")]
    InvalidId,

    #[display(fmt = "Invalid Input")]
    InvalidInput,

    #[display(fmt = "Internal Server Error")]
    InternalServerError,
}

#[derive(Debug, Display, Error, Serialize)]
#[display(fmt = "{}: {}", code, msg)]
pub struct ClientError {
    code: ErrorCodes,
    msg: String,
}

impl ClientError {
    pub fn new(code: ErrorCodes, msg: String) -> ClientError {
        ClientError { code, msg }
    }
}

impl error::ResponseError for ClientError {
    fn status_code(&self) -> StatusCode {
        match self.code {
            ErrorCodes::InvalidId => StatusCode::NOT_FOUND,
            ErrorCodes::InvalidInput => StatusCode::BAD_REQUEST,
            ErrorCodes::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(self)
    }
}
