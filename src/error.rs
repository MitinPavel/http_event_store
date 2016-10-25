use hyper;
use expected_version::ExpectedVersion;

#[derive(Debug)]
pub enum HesError { // `Hes` stands for HttpEventStore
    UserError(UserErrorKind),
    //LogicError,
    TransientFailure
}

#[derive(Debug)]
pub enum UserErrorKind {
    EventNumberMismatch(Option<ExpectedVersion>),
    StreamNotFound,
    BadRequest(hyper::client::Response),
    UnexpectedResponse(hyper::client::Response),
    Http(hyper::error::Error)
}

impl From<hyper::error::Error> for HesError {
    fn from(err: hyper::error::Error) -> HesError {
        HesError::UserError(UserErrorKind::Http(err))
    }
}
