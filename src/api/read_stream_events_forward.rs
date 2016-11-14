use hyper::Client as HyperClient;
use hyper::client::Response as HyperResponse;
use hyper::header::{Headers, Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;
use std::io::Read;
use serde_json;

use Stream;
use error::ApiError;
use connection::ConnectionInfo;
use api::ESResolveLinkTos;

pub struct Reader<'a> {
    connection_info: &'a ConnectionInfo,
    http_client: &'a HyperClient,
}

impl<'a> Reader<'a> {
    pub fn new(connection_info: &'a ConnectionInfo, http_client: &'a HyperClient) -> Reader<'a> {
        Reader { connection_info: connection_info, http_client: http_client }
    }

    pub fn read_stream_events_forward(&self,
                                      stream_name: &str,
                                      start: u32,
                                      count: u32,
                                      resolve_link_tos: bool)
                                      -> Result<Stream, ApiError> {
        let response = try!(self.http_client.get(&self.url(stream_name, start, count))
            .headers(build_headers(resolve_link_tos))
            .send());

        to_result(response)
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

fn to_result(response: HyperResponse) -> Result<Stream, ApiError> {
    match response.status {
        StatusCode::Ok => read_stream(response),
        StatusCode::NotFound => Err(ApiError::StreamNotFound),
        StatusCode::Gone => Err(ApiError::StreamDeleted),
        _ => Err(ApiError::UnexpectedResponse(response))
    }
}

fn read_stream(mut response: HyperResponse) -> Result<Stream, ApiError> {
    let mut body = String::new();
    try!(response.read_to_string(&mut body));
    let stream: Stream = try!(serde_json::from_str(&body));

    Ok(stream)
}
