use std::error::Error;
use std::io;
use hyper;
use serde_json;
use expected_version::ExpectedVersion;

#[derive(Debug)]
pub enum HesError { // `Hes` stands for HttpEventStore
    UserError(UserErrorKind),
    LogicError, //TODO Introduce 1. InternalError (for bugs in the library itself) and 2. ClientError (for incorrect use of the library).
    TransientFailure
}

#[derive(Debug)]
pub enum UserErrorKind {
    EventNumberMismatch(Option<ExpectedVersion>),
    StreamNotFound,
    Unexpected
}

impl From<hyper::error::Error> for HesError {
    fn from(err: hyper::error::Error) -> HesError {
        //TODO Substitute a stub below.
        HesError::LogicError
    }
}
