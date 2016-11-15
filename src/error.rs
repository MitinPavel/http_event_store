use hyper;
use serde_json;
use std::io;
use expected_version::ExpectedVersion;

#[derive(Debug)]
pub enum ApiError {
    WrongExpectedEventNumber(Option<ExpectedVersion>),
    StreamNotFound,
    StreamDeleted,

    Restful(hyper::client::Response),
    Http(hyper::error::Error),
    Json(serde_json::Error),
    Io(io::Error),
}

impl From<hyper::error::Error> for ApiError {
    fn from(err: hyper::error::Error) -> ApiError {
        ApiError::Http(err)
    }
}

impl From<io::Error> for ApiError {
    fn from(err: io::Error) -> ApiError {
        ApiError::Io(err)
    }
}

impl From<serde_json::Error> for ApiError {
 fn from(err: serde_json::Error) -> ApiError {
        ApiError::Json(err)
    }
}
