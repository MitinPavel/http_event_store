use std::result;

//use std::error::Error;

use std::io;
use hyper;
use serde_json;

#[derive(Debug)]
pub struct Error {
    repr: Repr,
}

#[derive(Debug)]
enum Repr {
    EsError(String),
    ServerError(String),
    IoError(io::Error),
    HttpError(hyper::error::Error),
    JsonError(serde_json::error::Error)
}

pub type Result<T> = result::Result<T, Error>;
