use hyper::Client;
use hyper::Result as HyperResult;
use hyper::client::Response as HyperResponse;
use hyper::status::StatusCode;

use types::Result;
use error::HesError;
use error::UserErrorKind;
use connection::ConnectionInfo;

pub struct Deleter<'a> {
   connection_info: &'a ConnectionInfo,
}

impl<'a> Deleter<'a> {
    pub fn new(connection_info: &'a ConnectionInfo) -> Deleter {
        Deleter { connection_info: connection_info }
    }

    pub fn delete(&self, stream_name: &str) -> Result<()> {
        let client = Client::default();

        let result = client.delete(&self.url(stream_name)).send();

        self.handle_result(result)
    }

    //TODO Handle Stream Already Deleted
    fn handle_result(&self, result: HyperResult<HyperResponse>) -> Result<()> {
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

    fn url(&self, stream_name: &str) -> String {
        format!("http://{}:{}/streams/{}",
                self.connection_info.host,
                self.connection_info.port,
                stream_name)
    }
}
