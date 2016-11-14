use hyper;
use serde_json;
use std::io;
use expected_version::ExpectedVersion;

#[derive(Debug)]
pub enum ApiError {
    UserError(UserErrorKind),
    LogicError(LogicErrorKind),
    //TransientFailure,
}

#[derive(Debug)]
pub enum UserErrorKind {
    WrongExpectedEventNumber(Option<ExpectedVersion>),
    StreamNotFound,
    StreamDeleted,
    BadRequest(hyper::client::Response),
    UnexpectedResponse(hyper::client::Response),
}

#[derive(Debug)]
pub enum LogicErrorKind {
    Json(serde_json::Error),
    Io(io::Error),
    Http(hyper::error::Error),
}

impl From<hyper::error::Error> for ApiError {
    fn from(err: hyper::error::Error) -> ApiError {
        ApiError::LogicError(LogicErrorKind::Http(err))
    }
}

impl From<io::Error> for ApiError {
    fn from(err: io::Error) -> ApiError {
        ApiError::LogicError(LogicErrorKind::Io(err))
    }
}

impl From<serde_json::Error> for ApiError {
 fn from(err: serde_json::Error) -> ApiError {
        ApiError::LogicError(LogicErrorKind::Json(err))
    }
}
