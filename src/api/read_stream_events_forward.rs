use hyper::Client;
use hyper::header::{Headers, Accept, qitem};
use hyper::client::Response as HyperResponse;
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;
use std::io::Read;
use serde_json;

use Stream;
use types::Result;
use error::HesError;
use error::UserErrorKind;

use api::ESResolveLinkTos;

pub struct Reader {}

impl Reader {
    pub fn read_stream_events_forward(&self, stream_name: &str, start: u32, count: u32, resolve_link_tos: bool) -> Result<Stream> {
        let client = Client::default();

        let mut headers = Headers::new();
        headers.set(
            Accept(vec![
            qitem(Mime(TopLevel::Application,
                   SubLevel::Ext("vnd.eventstore.atom+json".to_owned()), vec![]))]));
        headers.set(ESResolveLinkTos(resolve_link_tos));

        let url = format!("http://127.0.0.1:2113/streams/{}/{}/forward/{}?embed=body",
                          stream_name,
                          start,
                          count);

        let mut response = try!(client.get(&url)
            .headers(headers)
            .send());
        match response.status {
            StatusCode::Ok => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap(); //TODO Handle Result.
                let stream: Stream = serde_json::from_str(&body).unwrap();
                Ok(stream)
            },
            StatusCode::NotFound => {
                Err(HesError::UserError(UserErrorKind::StreamNotFound))
            },
            StatusCode::Gone => {
                Err(HesError::UserError(UserErrorKind::StreamDeleted))
            },
            _ => {
                self.panic_showing(&response)
            }
        }
    }

    //TODO Duplication with StramAppender
    fn panic_showing(&self, response: &HyperResponse) -> ! {
        panic!("hyper::status::StatusCode {} Response: {:?}", response.status, response)
    }
}
