use hyper;
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
    BadRequest(hyper::client::Response),
    Unexpected
}

impl From<hyper::error::Error> for HesError {
    fn from(err: hyper::error::Error) -> HesError {
        HesError::UserError(UserErrorKind::Unexpected) //TODO Capture hyper::error::Error
    }
}
