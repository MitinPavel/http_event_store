use std::result::Result as StdResult;
use hyper::Client;
use hyper::Result as HyperResult;
use hyper::client::Response as HyperResponse;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;

use event::Event;
use types::Result;
use expected_version::ExpectedVersion;
use error::HesError;
use error::UserErrorKind;
use connection::ConnectionInfo;

use api::ESCurrentVersion;
use api::ESExpectedVersion;

const WRONG_EXPECTED_EVENT_NUMBER: &'static str = "Wrong expected EventNumber";
const STREAM_DELETED: &'static str = "Stream deleted";

pub struct Appender<'a> {
    connection_info: &'a ConnectionInfo,
}

impl<'a> Appender<'a> {
    pub fn new(connection_info: &'a ConnectionInfo) -> Appender {
        Appender { connection_info: connection_info }
    }

    pub fn append_to_stream(&self, stream_name: &str, expected_version: ExpectedVersion, events: Vec<Event>) -> Result<()> {
        let client = Client::default();

        let result = client.post(&self.url(stream_name))
            .headers(build_headers(expected_version))
            .body(&request_body(events))
            .send();

        to_hes_result(result)
    }

    fn url(&self, stream_name: &str) -> String {
        format!("http://{}:{}/streams/{}",
                self.connection_info.host,
                self.connection_info.port,
                stream_name)
    }
}

fn build_headers(expected_version: ExpectedVersion) -> Headers {
    let mut headers = Headers::new();
    headers.set(
        ContentType(Mime(TopLevel::Application, SubLevel::Ext("vnd.eventstore.events+json".to_owned()), vec![]))
    );
    headers.set(ESExpectedVersion(expected_version.into()));

    headers
}

fn request_body(events: Vec<Event>) -> String {
    let events_as_json: Vec<String> = events.iter().map(|e| {
        format!(r#"{{
                      "eventType": "{}",
                      "eventId": "{}",
                      "data": {}
                    }}"#,
                e.event_type.to_string(),
                e.event_id.hyphenated().to_string(),
                e.data.clone().unwrap()) //TODO Eliminate `clone` and deal with `unwrap`.
    }).collect::<_>();

    format!("[{}]", events_as_json.join(","))
}

fn to_hes_result(result: HyperResult<HyperResponse>) -> Result<()> {
    match result {
        Ok(response) => {
            match response.status {
                StatusCode::Created => Ok(()),
                _ => stream_deleted_error(response)
                    .and_then(event_number_mismatch_error)
                    .map_err(|kind| HesError::UserError(kind))
                    .and_then(default_error)
            }
        },
        Err(err) => Err(HesError::UserError(UserErrorKind::Http(err)))
    }
}

fn stream_deleted_error(response: HyperResponse) -> StdResult<HyperResponse, UserErrorKind> {
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

fn event_number_mismatch_error(response: HyperResponse) -> StdResult<HyperResponse, UserErrorKind> {
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

fn default_error(response: HyperResponse) -> Result<()> {
    Err(HesError::UserError(UserErrorKind::UnexpectedResponse(response)))
}

fn expected_version(response: &HyperResponse) -> Option<ExpectedVersion> {
    response.headers
        .get::<ESCurrentVersion>()
        .and_then(|header| Some(ExpectedVersion::from(header.to_string())))
}