use std::error::Error;
use std::io;
use hyper;
use serde_json;
use expected_version::ExpectedVersion;

#[derive(Debug)]
pub enum ClientError {
    EventNumberMismatch(ExpectedVersion),
    StreamNotFound,
    Unexpected
}

#[derive(Debug)]
pub enum HesError {
    ClientError(ClientError),
    ServerError(String),
    IoError(io::Error),
    HttpError(hyper::error::Error),
    JsonError(serde_json::error::Error)
}

impl From<hyper::error::Error> for HesError {
    fn from(err: hyper::error::Error) -> HesError {
        HesError::HttpError(err)
    }
}
