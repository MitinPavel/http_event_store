use hyper::Client as HyperClient;
use hyper::client::Response as HyperResponse;
use hyper::status::StatusCode;
use hyper::header::Headers;

use error::ApiError;
use connection::ConnectionInfo;
use expected_version::ExpectedVersion;
use api::ESExpectedVersion;
use api::ESHardDelete;
use api::to_error::*;

pub struct Deleter<'a> {
    connection_info: &'a ConnectionInfo,
    http_client: &'a HyperClient,
}

impl<'a> Deleter<'a> {
    pub fn new(connection_info: &'a ConnectionInfo, http_client: &'a HyperClient) -> Deleter<'a> {
        Deleter { connection_info: connection_info, http_client: http_client }
    }

    pub fn delete(&self, stream_name: &str, expected_version: ExpectedVersion)
                  -> Result<(), ApiError> {
        self.do_delete(stream_name, expected_version, false)
    }

    pub fn hard_delete(&self, stream_name: &str, expected_version: ExpectedVersion)
                       -> Result<(), ApiError> {
        self.do_delete(stream_name, expected_version, true)
    }

    fn do_delete(&self,
                 stream_name: &str,
                 expected_version: ExpectedVersion,
                 is_hard: bool)
                 -> Result<(), ApiError> {
        let response = self.http_client.delete(&self.url(stream_name))
            .headers(build_headers(expected_version, is_hard))
            .send()?;

        to_result(response, stream_name)
    }

    fn url(&self, stream_name: &str) -> String {
        format!("http://{}:{}/streams/{}",
                self.connection_info.host,
                self.connection_info.port,
                stream_name)
    }
}

fn build_headers(expected_version: ExpectedVersion, is_hard: bool) -> Headers {
    let mut headers = Headers::new();
    headers.set(ESExpectedVersion(expected_version.into()));
    headers.set(ESHardDelete(is_hard));

    headers
}

fn to_result(response: HyperResponse, stream_name: &str) -> Result<(), ApiError> {
    match response.status {
        StatusCode::NoContent => Ok(()),
        _ => check_stream_deleted(response, stream_name)
            .and_then(check_wrong_expected_event_number)
            .and_then(default_error)
    }
}
