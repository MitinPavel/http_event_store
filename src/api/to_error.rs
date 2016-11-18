use hyper::client::Response as HyperResponse;
use hyper::status::StatusCode;

use expected_version::ExpectedVersion;
use error::ApiError;
use api::ESCurrentVersion;

const WRONG_EXPECTED_EVENT_NUMBER: &'static str = "Wrong expected EventNumber";
const STREAM_DELETED: &'static str = "Stream deleted";

pub fn default_error(response: HyperResponse) -> Result<(), ApiError> {
    Err(ApiError::Restful(response))
}

pub fn check_stream_deleted(response: HyperResponse, stream_name: &str) -> Result<HyperResponse, ApiError> {
    match response.status {
        StatusCode::Gone => {
            if { response.status_raw().1 == STREAM_DELETED } {
                Err(ApiError::StreamDeleted(stream_name.into()))
            } else {
                Err(ApiError::Restful(response))
            }
        },
        _ => Ok(response)
    }
}

pub fn check_wrong_expected_event_number(response: HyperResponse)
                                   -> Result<HyperResponse, ApiError> {
    match response.status {
        StatusCode::BadRequest => {
            if { response.status_raw().1 == WRONG_EXPECTED_EVENT_NUMBER } {
                Err(ApiError::WrongExpectedEventNumber(expected_version(&response)))
            } else {
                Err(ApiError::Restful(response))
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
