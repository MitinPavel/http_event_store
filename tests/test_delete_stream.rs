extern crate serde;
extern crate serde_json;
extern crate time;
extern crate uuid;
extern crate http_event_store as hes;

#[macro_use]
mod support;

use support::task_domain::*;

use hes::event::Event;
use hes::client::Client;
use hes::expected_version::ExpectedVersion;

#[test]
fn smoke_test() {
    let events: Vec<Event> = vec![
        TaskCreated { name: "Created".to_string(),
                      event_id: uuid::Uuid::parse_str("baca1a30-b6f1-470b-b68e-f79338020327").unwrap() }.into()
    ];

    let client = Client::default();
    let stream_name = test_stream_name();
    client.append_to_stream(&stream_name, ExpectedVersion::NoStream, events).unwrap();

    assert!(client.delete_stream(&stream_name).is_ok());
}

fn test_stream_name() -> String {
    format!("task-{}", uuid::Uuid::new_v4().simple())
}
