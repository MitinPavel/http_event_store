use hyper::Client;
use hyper::header::{Headers, Accept, ContentType, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use std::io::Read;
use serde_json;

use Stream;
use error::Result;

pub struct Api {}

impl Api {
    pub fn append_to_stream(&self, stream_name: &str, expectedVersion: u64, events_json: String) {

        let client = Client::new();

        let mut headers = Headers::new();

        headers.set(
            ContentType(Mime(TopLevel::Application, SubLevel::Ext("vnd.eventstore.events+json".to_owned()), vec![]))
        );

        let url = format!("http://127.0.0.1:2113/streams/{}", stream_name);

        let mut response = client.post(&url)
            .headers(headers)
            .body(&events_json)
            .send()
            .unwrap();

        println!("Result: {:?}", response);
    }

    pub fn read_stream_events_forward(&self, stream_name: &str, start: u32, count: u32, resolve_link_tos: bool) -> Result<Stream> {
        let client = Client::new();
        let mut headers = Headers::new();
        headers.set(
            Accept(vec![
            qitem(Mime(TopLevel::Application,
                   SubLevel::Ext("vnd.eventstore.atom+json".to_owned()), vec![]))]));

        let url = format!("http://127.0.0.1:2113/streams/{}?embed=body", stream_name);

        let mut response = client
            .get(&url)
            .headers(headers)
            .send().unwrap();

        let mut body = String::new();
        response.read_to_string(&mut body);
        let stream: Stream = serde_json::from_str(&body).unwrap();

        Ok(stream)
    }

    pub fn get(&self) {
        let client = Client::new();
        let mut headers = Headers::new();
        headers.set(
            Accept(vec![
            qitem(Mime(TopLevel::Application,
                   SubLevel::Ext("vnd.eventstore.atom+json".to_owned()), vec![]))]));
        let mut response = client
            .get("http://127.0.0.1:2113/streams/newstream?embed=body")
            .headers(headers)
            .send().unwrap();

        let mut body = String::new();
        response.read_to_string(&mut body);
        let stream: Stream = serde_json::from_str(&body).unwrap();

        // Result: Response { status: Ok, headers: Headers { ETag: "0;248368668", Keep-Alive: timeout=15,max=100, Access-Control-Allow-Headers: Content-Type, X-Requested-With, X-Forwarded-Host, X-PINGOTHER, Authorization, ES-LongPoll, ES-ExpectedVersion, ES-EventId, ES-EventType, ES-RequiresMaster, ES-HardDelete, ES-ResolveLinkTo, Content-Type: application/vnd.eventstore.atom+json; charset=utf-8, Server: Mono-HTTPAPI/1.0, Vary: Accept, Access-Control-Allow-Methods: POST, DELETE, GET, OPTIONS, Access-Control-Expose-Headers: Location, ES-Position, ES-CurrentVersion, Content-Length: 1260, Cache-Control: max-age=0, no-cache, must-revalidate, Date: Tue, 06 Sep 2016 20:51:52 GMT, Access-Control-Allow-Origin: *, }, version: Http11, url: "http://127.0.0.1:2113/streams/newstream", status_raw: RawStatus(200, "OK"), message: Http11Message { is_proxied: false, method: None, stream: Wrapper { obj: Some(Reading(SizedReader(remaining=0))) } } }
        // Body: "{\n  \"title\": \"Event stream \'newstream\'\",\n  \"id\": \"http://127.0.0.1:2113/streams/newstream\",\n  \"updated\": \"2016-09-06T18:14:50.042706Z\",\n  \"streamId\": \"newstream\",\n  \"author\": {\n    \"name\": \"EventStore\"\n  },\n  \"headOfStream\": true,\n  \"selfUrl\": \"http://127.0.0.1:2113/streams/newstream\",\n  \"eTag\": \"0;248368668\",\n  \"links\": [\n    {\n      \"uri\": \"http://127.0.0.1:2113/streams/newstream\",\n      \"relation\": \"self\"\n    },\n    {\n      \"uri\": \"http://127.0.0.1:2113/streams/newstream/head/backward/20\",\n      \"relation\": \"first\"\n    },\n    {\n      \"uri\": \"http://127.0.0.1:2113/streams/newstream/1/forward/20\",\n      \"relation\": \"previous\"\n    },\n    {\n      \"uri\": \"http://127.0.0.1:2113/streams/newstream/metadata\",\n      \"relation\": \"metadata\"\n    }\n  ],\n  \"entries\": [\n    {\n      \"title\": \"0@newstream\",\n      \"id\": \"http://127.0.0.1:2113/streams/newstream/0\",\n      \"updated\": \"2016-09-06T18:14:50.042706Z\",\n      \"author\": {\n        \"name\": \"EventStore\"\n      },\n      \"summary\": \"event-type\",\n      \"links\": [\n        {\n          \"uri\": \"http://127.0.0.1:2113/streams/newstream/0\",\n          \"relation\": \"edit\"\n        },\n        {\n          \"uri\": \"http://127.0.0.1:2113/streams/newstream/0\",\n          \"relation\": \"alternate\"\n        }\n      ]\n    }\n  ]\n}"
//        println!("Result: {:?}", response);
//        println!("Stream: {:?}", stream);
//        println!("value.headOfStream: {:?}", stream.head_of_stream);
    }
}
