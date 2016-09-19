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
