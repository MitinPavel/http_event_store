use hyper::Client;
use hyper::header::{Headers, Accept, ContentType, qitem};
use hyper::Result as HyperResult;
use hyper::client::Response as HyperResponse;
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;
use hyper::http::RawStatus;
use std::io::Read;
use serde_json;
use std::borrow::Cow;

use Stream;
use event::Event;
use types::Result;
use expected_version::ExpectedVersion;
use error::HesError;
use error::UserErrorKind;

header! { (ESCurrentVersion, "ES-CurrentVersion") => [String] }
header! { (ESExpectedVersion, "ES-ExpectedVersion") => [String] }
header! { (ESResolveLinkTos, "ES-ResolveLinkTos") => [bool] }

pub struct Api {}

impl Api {
    pub fn append_to_stream(&self, stream_name: &str, expected_version: ExpectedVersion, events: Vec<Event>) -> Result<()> {
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
        let events_json: String = format!("[{}]", events_as_json.join(","));

        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(
            ContentType(Mime(TopLevel::Application, SubLevel::Ext("vnd.eventstore.events+json".to_owned()), vec![]))
        );
        headers.set(ESExpectedVersion(expected_version.into()));

        let url = format!("http://127.0.0.1:2113/streams/{}", stream_name);

        let result: HyperResult<HyperResponse> = client.post(&url)
            .headers(headers)
            .body(&events_json)
            .send();

        match result {
            Ok(response) => {


                match response.status {
                    StatusCode::Created => Ok(()),
                    StatusCode::BadRequest => {
                        //      Response { status: BadRequest, headers:
                        //        Headers { Access-Control-Allow-Headers: Content-Type, X-Requested-With,
                        //                  X-Forwarded-Host, X-PINGOTHER, Authorization, ES-LongPoll,
                        //                  ES-ExpectedVersion, ES-EventId, ES-EventType, ES-RequiresMaster,
                        //                  ES-HardDelete, ES-ResolveLinkTo,
                        //                  Content-Type: text/plain; charset=utf-8,
                        //                  Content-Length: 0,
                        //                  Date: Wed, 12 Oct 2016 16:24:28 GMT,
                        //                  Connection: close,
                        //                  Access-Control-Allow-Origin: *,
                        // --->             ES-CurrentVersion: -1,
                        //                  Access-Control-Expose-Headers: Location, ES-Position, ES-CurrentVersion,
                        //                  Access-Control-Allow-Methods: POST, DELETE, GET, OPTIONS, Server: Mono-HTTPAPI/1.0, },
                        //        version: Http11,
                        //        url: "http://127.0.0.1:2113/streams/task-c0340f57a914468ea6b48f7dff3519dc",
                        // --->   status_raw: RawStatus(400, "Wrong expected EventNumber"),
                        //        message: Http11Message { is_proxied: false, method: None, stream: Wrapper { obj: Some(Reading(SizedReader(remaining=0))) } } }
                        match response.status_raw() {
                            &RawStatus(400, ref reason_phrase) => {
                                if reason_phrase == "Wrong expected EventNumber" {
                                    match response.headers.get::<ESCurrentVersion>() {
                                        Some(version) => {
                                          return Err(HesError::UserError(UserErrorKind::EventNumberMismatch(ExpectedVersion::from(version.to_string()))))
                                        },
                                        None => panic!("Cannot find ESCurrentVersion in response: {:?}", response) //TODO
                                    };
                                } else {
                                  self.panic_showing(&response) //TODO Return 'generic' BadRequest
                                }
                            },
                            _ => self.panic_showing(&response)
                        }
                    },
                    _ => self.panic_showing(&response)
                }
            },
            Err(err) => Err(HesError::UserError(UserErrorKind::Unexpected))
        }
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
                Err(HesError::UserError(UserErrorKind::StreamNotFound))
            },
            _ => {
                self.panic_showing(&response)
            }
        }
    }

    fn panic_showing(&self, response: &HyperResponse) -> ! {
        panic!("hyper::status::StatusCode {} Response: {:?}", response.status, response)
    }
}
