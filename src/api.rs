use hyper::Client;
use hyper::header::{Headers, Accept, ContentType, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;
use std::io::Read;
use serde_json;

use Stream;
use event::Event;
use types::Result;
use expected_version::ExpectedVersion;
use error::HesError;

// "ES-ExpectedVersion: 3"
header! { (ESExpectedVersion, "ES-ExpectedVersion") => [String] }
header! { (ESResolveLinkTos, "ES-ResolveLinkTos") => [bool] }

pub struct Api {}

impl Api {
    pub fn append_to_stream(&self, stream_name: &str, expected_version: ExpectedVersion, events: Vec<Event>) {
        let events_as_json : Vec<String> = events.iter().map(|e| {
            format!(r#"{{
                      "eventType": "{}",
                      "eventId": "{}",
                      "data": {}
                    }}"#,
                    e.event_type.to_string(),
                    e.event_id.hyphenated().to_string(),
                    e.data.clone().unwrap()) //TODO Eliminate `clone` and deal with `unwrap`.
        }).collect::<Vec<String>>();
        let events_json : String = format!("[{}]", events_as_json.join(","));

        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(
            ContentType(Mime(TopLevel::Application, SubLevel::Ext("vnd.eventstore.events+json".to_owned()), vec![]))
        );
        headers.set(ESExpectedVersion(expected_version.into()));

        let url = format!("http://127.0.0.1:2113/streams/{}", stream_name);

        let mut response = client.post(&url)
            .headers(headers)
            .body(&events_json)
            .send()
            .unwrap();

//        println!("Result: {:?}", response);
    }

    pub fn read_stream_events_forward(&self, stream_name: &str, start: u32, count: u32, resolve_link_tos: bool) -> Result<Stream> {
        let client = Client::new();

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
                response.read_to_string(&mut body);
                let stream: Stream = serde_json::from_str(&body).unwrap();
                Ok(stream)
            },
            StatusCode::NotFound => {
                Err(HesError::ClientError(format!("Stream {} NotFound", stream_name)))
            },
            _ => {
                panic!("hyper::status::StatusCode {}", response.status)
            }
        }
    }
}
