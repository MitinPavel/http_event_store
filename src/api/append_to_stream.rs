use hyper::Client;
use hyper::header::{Headers, ContentType};
use hyper::Result as HyperResult;
use hyper::client::Response as HyperResponse;
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;

use event::Event;
use types::Result;
use expected_version::ExpectedVersion;
use error::HesError;
use error::UserErrorKind;
use connection::ConnectionInfo;

use api::ESCurrentVersion;
use api::ESExpectedVersion;

const WRONG_EXPECTED_EVENT_NUMBER: &'static str = "Wrong expected EventNumber";

pub struct Appender<'a> {
   connection_info: &'a ConnectionInfo,
}

impl<'a> Appender<'a> {
    pub fn new(connection_info: &'a ConnectionInfo) -> Appender {
        Appender { connection_info: connection_info }
    }

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

        let client = Client::default();

        let mut headers = Headers::new();
        headers.set(
            ContentType(Mime(TopLevel::Application, SubLevel::Ext("vnd.eventstore.events+json".to_owned()), vec![]))
        );
        headers.set(ESExpectedVersion(expected_version.into()));

        let result: HyperResult<HyperResponse> = client.post(&self.url(stream_name))
            .headers(headers)
            .body(&events_json)
            .send();

        match result {
            Ok(response) => {
                match response.status {
                    StatusCode::Created => Ok(()),
                    StatusCode::BadRequest => self.handle_bad_request_on_append(response),
                    _ => self.panic_showing(&response)
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

    fn handle_bad_request_on_append(&self, response: HyperResponse) -> Result<()> {
        // RawStatus(400, ref reason_phrase) => {
        //   Response { status: BadRequest, headers:
        //     Headers { Access-Control-Allow-Headers: Content-Type, X-Requested-With,
        //               X-Forwarded-Host, X-PINGOTHER, Authorization, ES-LongPoll,
        //               ES-ExpectedVersion, ES-EventId, ES-EventType, ES-RequiresMaster,
        //               ES-HardDelete, ES-ResolveLinkTo,
        //               Content-Type: text/plain; charset=utf-8,
        //               Content-Length: 0,
        //               Date: Wed, 12 Oct 2016 16:24:28 GMT,
        //               Connection: close,
        //               Access-Control-Allow-Origin: *,
        // -->           ES-CurrentVersion: -1,
        //               Access-Control-Expose-Headers: Location, ES-Position, ES-CurrentVersion,
        //               Access-Control-Allow-Methods: POST, DELETE, GET, OPTIONS, Server: Mono-HTTPAPI/1.0, },
        //     version: Http11,
        //     url: "http://127.0.0.1:2113/streams/task-c0340f57a914468ea6b48f7dff3519dc",
        // --> status_raw: RawStatus(400, "Wrong expected EventNumber"),
        //     message: Http11Message { is_proxied: false, method: None, stream: Wrapper { obj: Some(Reading(SizedReader(remaining=0))) } } }
        let is_wrong_expected_number = { response.status_raw().1 == WRONG_EXPECTED_EVENT_NUMBER };

        let error_kind = if is_wrong_expected_number {
            let version = self.expected_version(&response);
            UserErrorKind::EventNumberMismatch(version)
        } else {
            UserErrorKind::BadRequest(response)
        };

        Err(HesError::UserError(error_kind))
    }

    fn expected_version(&self, response: &HyperResponse) -> Option<ExpectedVersion> {
        response.headers.get::<ESCurrentVersion>()
            .and_then(|header| Some(ExpectedVersion::from(header.to_string())))
    }

    fn panic_showing(&self, response: &HyperResponse) -> ! {
        panic!("hyper::status::StatusCode {} Response: {:?}", response.status, response)
    }
}
