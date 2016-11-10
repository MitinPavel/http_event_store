use hyper::Client;
use hyper::client::Response as HyperResponse;
use hyper::header::{Headers, Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;
use std::io::Read;
use serde_json;

use Stream;
use types::Result;
use error::HesError;
use error::UserErrorKind;
use connection::ConnectionInfo;
use api::ESResolveLinkTos;

pub struct Reader<'a> {
    connection_info: &'a ConnectionInfo,
}

impl<'a> Reader<'a> {
    pub fn new(connection_info: &'a ConnectionInfo) -> Reader {
        Reader { connection_info: connection_info }
    }

    pub fn read_stream_events_forward(&self,
                                      stream_name: &str,
                                      start: u32,
                                      count: u32,
                                      resolve_link_tos: bool)
                                      -> Result<Stream> {
        let http_client = Client::default();

        let response = try!(http_client.get(&self.url(stream_name, start, count))
            .headers(build_headers(resolve_link_tos))
            .send());

        to_hes_result(response)
    }

    fn url(&self, stream_name: &str, start: u32, count: u32) -> String {
        format!("http://{}:{}/streams/{}/{}/forward/{}?embed=body",
                self.connection_info.host,
                self.connection_info.port,
                stream_name,
                start,
                count)
    }
}

fn build_headers(resolve_link_tos: bool) -> Headers {
    let mut headers = Headers::new();
    headers.set(
        Accept(vec![
        qitem(Mime(TopLevel::Application,
                   SubLevel::Ext("vnd.eventstore.atom+json".to_owned()), vec![]))]));
    headers.set(ESResolveLinkTos(resolve_link_tos));

    headers
}

fn to_hes_result(mut response: HyperResponse) -> Result<Stream> {
    match response.status {
        StatusCode::Ok => {
            let mut body = String::new();
            try!(response.read_to_string(&mut body));
            let stream: Stream = try!(serde_json::from_str(&body));
            Ok(stream)
        },
        StatusCode::NotFound => {
            Err(HesError::UserError(UserErrorKind::StreamNotFound))
        },
        StatusCode::Gone => {
            Err(HesError::UserError(UserErrorKind::StreamDeleted))
        },
        _ => {
            Err(HesError::UserError(UserErrorKind::UnexpectedResponse(response)))
        }
    }
}
