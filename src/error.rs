use hyper;
use serde_json;
use std::io;
use expected_version::ExpectedVersion;

#[derive(Debug)]
pub enum HesError { // `Hes` stands for HttpEventStore
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

impl From<hyper::error::Error> for HesError {
    fn from(err: hyper::error::Error) -> HesError {
        HesError::LogicError(LogicErrorKind::Http(err))
    }
}

impl From<io::Error> for HesError {
    fn from(err: io::Error) -> HesError {
        HesError::LogicError(LogicErrorKind::Io(err))
    }
}

impl From<serde_json::Error> for HesError {
 fn from(err: serde_json::Error) -> HesError {
        HesError::LogicError(LogicErrorKind::Json(err))
    }
}
