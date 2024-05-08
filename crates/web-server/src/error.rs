use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use db::{DieselError, InteractError, PoolError};
use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InternalServerError(String),
    Database(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InternalServerError(ref cause) => write!(f, "Internal Server Error: {}", cause),
            Error::Database(ref cause) => write!(f, "Database Error: {}", cause),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            Error::InternalServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
        };

        format!("status = {}, message = {}", status, error_message).into_response()
    }
}

impl From<PoolError<DieselError>> for Error {
    fn from(err: PoolError<DieselError>) -> Self {
        Error::Database(err.to_string())
    }
}

impl From<InteractError> for Error {
    fn from(err: InteractError) -> Self {
        Error::Database(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::InternalServerError(err.to_string())
    }
}

impl std::error::Error for Error {}
