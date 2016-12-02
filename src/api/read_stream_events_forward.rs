use hyper::Client as HyperClient;
use hyper::client::Response as HyperResponse;
use hyper::header::{Headers, Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;
use std::io::Read;
use serde;
use serde_json;

use read::Stream;
use read::EmbedLevel;
use error::ApiError;
use connection::ConnectionInfo;
use api::ESResolveLinkTos;

pub struct StreamReader<'a> {
    connection_info: &'a ConnectionInfo,
    http_client: &'a HyperClient,
}

impl<'a> StreamReader<'a> {
    pub fn new(connection_info: &'a ConnectionInfo, http_client: &'a HyperClient) -> StreamReader<'a> {
        StreamReader { connection_info: connection_info, http_client: http_client }
    }

    pub fn read_forward<E: serde::Deserialize + EmbedLevel>(&self,
                                                            stream_name: &str,
                                                            start: i32,
                                                            count: i32,
                                                            resolve_link_tos: bool)
                                                            -> Result<Stream<E>, ApiError> {
        let response = self.http_client.get(&self.url::<E>(stream_name, start, count))
            .headers(build_headers(resolve_link_tos))
            .send()?;

        to_result::<E>(response, stream_name)
    }

    fn url<E: EmbedLevel>(&self, stream_name: &str, start: i32, count: i32) -> String {
        format!("http://{}:{}/streams/{}/{}/forward/{}?embed={}",
                self.connection_info.host,
                self.connection_info.port,
                stream_name,
                start,
                count,
                E::level())
    }
}

fn build_headers(resolve_link_tos: bool) -> Headers {
    let mut headers = Headers::new();
    headers.set(ESResolveLinkTos(resolve_link_tos));
    headers.set(
        Accept(vec![
        qitem(Mime(TopLevel::Application,
                   SubLevel::Ext("vnd.eventstore.atom+json".to_owned()), vec![]))]));

    headers
}

fn to_result<E: serde::Deserialize + EmbedLevel>(response: HyperResponse, stream_name: &str)
                                                 -> Result<Stream<E>, ApiError> {
    match response.status {
        StatusCode::Ok => read_stream::<E>(response),
        StatusCode::NotFound => Err(ApiError::StreamNotFound(stream_name.into())),
        StatusCode::Gone => Err(ApiError::StreamDeleted(stream_name.into())),
        _ => Err(ApiError::Restful(response))
    }
}

fn read_stream<E: serde::Deserialize + EmbedLevel>(mut response: HyperResponse)
                                                   -> Result<Stream<E>, ApiError> {
    let mut body = String::new();
    response.read_to_string(&mut body)?;
    let stream: Stream<E> = serde_json::from_str(&body)?;

    Ok(stream)
}
