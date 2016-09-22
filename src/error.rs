use std::error::Error;
use std::io;
use hyper;
use serde_json;

#[derive(Debug)]
pub enum HesError {
    ClientError(String),
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
