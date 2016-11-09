use hyper;
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
    EventNumberMismatch(Option<ExpectedVersion>),
    StreamNotFound,
    StreamDeleted,
    BadRequest(hyper::client::Response),
    UnexpectedResponse(hyper::client::Response),
    Http(hyper::error::Error),
}

#[derive(Debug)]
pub enum LogicErrorKind {
    Io(io::Error),
}

impl From<hyper::error::Error> for HesError {
    fn from(err: hyper::error::Error) -> HesError {
        HesError::UserError(UserErrorKind::Http(err))
    }
}

impl From<io::Error> for HesError {
    fn from(err: io::Error) -> HesError {
        HesError::LogicError(LogicErrorKind::Io(err))
    }
}
