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

pub struct Deleter<'a> {
   connection_info: &'a ConnectionInfo,
}

impl<'a> Deleter<'a> {
    pub fn new(connection_info: &'a ConnectionInfo) -> Deleter {
        Deleter { connection_info: connection_info }
    }

    pub fn delete(&self, stream_name: &str) -> Result<()> {
        self.do_delete(stream_name, false)
    }

    pub fn hard_delete(&self, stream_name: &str) -> Result<()> {
        self.do_delete(stream_name, true)
    }

    fn do_delete(&self, stream_name: &str, is_hard: bool) -> Result<()> {
        let client = Client::default();

        let result = client.delete(&self.url(stream_name))
            .headers(build_headers(is_hard))
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

fn build_headers(is_hard: bool) -> Headers {
    let mut headers = Headers::new();
    headers.set(ESExpectedVersion(ExpectedVersion::Any.into()));
    headers.set(ESHardDelete(is_hard));

    headers
}

//TODO Handle Stream Already Deleted
fn to_hes_result(result: HyperResult<HyperResponse>) -> Result<()> {
    match result {
        Ok(response) => {
            match response.status {
                StatusCode::NoContent => Ok(()),
                _ => Err(HesError::UserError(UserErrorKind::UnexpectedResponse(response)))
            }
        },
        Err(err) => Err(HesError::UserError(UserErrorKind::Http(err)))
    }
}
