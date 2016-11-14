use hyper::Client as HyperClient;
use hyper::client::Response as HyperResponse;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;

use event::Event;
use types::Result;
use expected_version::ExpectedVersion;
use error::HesError;
use connection::ConnectionInfo;

use api::ESExpectedVersion;
use api::to_error::*;

pub struct Appender<'a> {
    connection_info: &'a ConnectionInfo,
    http_client: &'a HyperClient,
}

impl<'a> Appender<'a> {
    pub fn new(connection_info: &'a ConnectionInfo, http_client: &'a HyperClient) -> Appender<'a> {
        Appender { connection_info: connection_info, http_client: http_client }
    }

    pub fn append<I>(&self, stream_name: &str,
                     expected_version: ExpectedVersion,
                     events: I) -> Result<()>
        where I: IntoIterator<Item = Event> {
        let response = try!(self.http_client.post(&self.url(stream_name))
            .headers(build_headers(expected_version))
            .body(&request_body(events))
            .send());

        to_result(response)
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
        ContentType(Mime(TopLevel::Application,
                         SubLevel::Ext("vnd.eventstore.events+json".to_owned()), vec![]))
    );
    headers.set(ESExpectedVersion(expected_version.into()));

    headers
}

fn request_body<I>(events: I) -> String where I: IntoIterator<Item = Event> {
    let events_as_json: Vec<String> = events.into_iter().map(|e| {
        let mut result: String = format!(r#"{{"eventType":"{}""#, e.event_type);

        if let Some(id) = e.event_id {
            let id_pair = &format!(r#","eventId":"{}""#, id);
            result.push_str(id_pair)
        }

        if let Some(ref data) = e.data {
            let data_pair = &format!(r#","data":{}"#, data);
            result.push_str(data_pair)
        }

        result.push_str(r#"}"#);

        result
    }).collect::<_>();

    format!("[{}]", events_as_json.join(","))
}

fn to_result(response: HyperResponse) -> Result<()> {
    match response.status {
        StatusCode::Created => Ok(()),
        _ => check_stream_deleted(response)
            .and_then(check_wrong_expected_event_number)
            .map_err(HesError::UserError)
            .and_then(default_error)
    }
}
