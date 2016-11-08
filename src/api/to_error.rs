use std::result::Result as StdResult;
use hyper::client::Response as HyperResponse;
use hyper::status::StatusCode;

use types::Result;
use expected_version::ExpectedVersion;
use error::HesError;
use error::UserErrorKind;
use api::ESCurrentVersion;

const WRONG_EXPECTED_EVENT_NUMBER: &'static str = "Wrong expected EventNumber";
const STREAM_DELETED: &'static str = "Stream deleted";

pub fn default_error(response: HyperResponse) -> Result<()> {
    Err(HesError::UserError(UserErrorKind::UnexpectedResponse(response)))
}

pub fn stream_deleted_error(response: HyperResponse) -> StdResult<HyperResponse, UserErrorKind> {
    match response.status {
        StatusCode::Gone => {
            if { response.status_raw().1 == STREAM_DELETED } {
                Err(UserErrorKind::StreamDeleted)
            } else {
                Err(UserErrorKind::UnexpectedResponse(response))
            }
        },
        _ => Ok(response)
    }
}

pub fn event_number_mismatch_error(response: HyperResponse)
                                   -> StdResult<HyperResponse, UserErrorKind> {
    match response.status {
        StatusCode::BadRequest => {
            if { response.status_raw().1 == WRONG_EXPECTED_EVENT_NUMBER } {
                let version = expected_version(&response);
                Err(UserErrorKind::EventNumberMismatch(version))
            } else {
                Err(UserErrorKind::BadRequest(response))
            }
        },
        _ => Ok(response)
    }
}

fn expected_version(response: &HyperResponse) -> Option<ExpectedVersion> {
    response.headers
        .get::<ESCurrentVersion>()
        .and_then(|header| Some(ExpectedVersion::from(header.to_string())))
}
