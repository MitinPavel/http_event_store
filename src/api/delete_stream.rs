use hyper::Client;
use hyper::Result as HyperResult;
use hyper::client::Response as HyperResponse;
use hyper::status::StatusCode;
use hyper::header::Headers;

use types::Result;
use error::HesError;
use error::UserErrorKind;
use connection::ConnectionInfo;
use expected_version::ExpectedVersion;
use api::ESExpectedVersion;
use api::ESHardDelete;
use api::to_error::*;

pub struct Deleter<'a> {
    connection_info: &'a ConnectionInfo,
}

impl<'a> Deleter<'a> {
    pub fn new(connection_info: &'a ConnectionInfo) -> Deleter {
        Deleter { connection_info: connection_info }
    }

    pub fn delete(&self, stream_name: &str, expected_version: ExpectedVersion) -> Result<()> {
        self.do_delete(stream_name, expected_version, false)
    }

    pub fn hard_delete(&self, stream_name: &str, expected_version: ExpectedVersion) -> Result<()> {
        self.do_delete(stream_name, expected_version, true)
    }

    fn do_delete(&self,
                 stream_name: &str,
                 expected_version: ExpectedVersion,
                 is_hard: bool)
                 -> Result<()> {
        let http_client = Client::default();

        let result = http_client.delete(&self.url(stream_name))
            .headers(build_headers(expected_version, is_hard))
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

fn build_headers(expected_version: ExpectedVersion, is_hard: bool) -> Headers {
    let mut headers = Headers::new();
    headers.set(ESExpectedVersion(expected_version.into()));
    headers.set(ESHardDelete(is_hard));

    headers
}

fn to_hes_result(result: HyperResult<HyperResponse>) -> Result<()> {
    match result {
        Ok(response) => {
            match response.status {
                StatusCode::NoContent => Ok(()),
                _ => stream_deleted_error(response)
                    .and_then(event_number_mismatch_error)
                    .map_err(|kind| HesError::UserError(kind))
                    .and_then(default_error)
            }
        },
        Err(err) => Err(HesError::UserError(UserErrorKind::Http(err)))
    }
}
