extern crate serde;
extern crate serde_json;
extern crate time;
extern crate uuid;
extern crate http_event_store as es;

mod support;
use support::*;

use es::event::Event;
use es::client::Client;
use es::expected_version::ExpectedVersion;

#[test]
fn it_appends_events_in_right_order() {
    let created_id = "baca1a30-b6f1-470b-b68e-f79338020327";
    let renamed_id = "cbad187b-2fd0-4ad2-b78b-80d83f1ff303";
    let events: Vec<Box<Event>> = vec![
        Box::new(TaskCreated { name: format!("Created {:?}", time::get_time()), event_id: uuid::Uuid::parse_str("baca1a30-b6f1-470b-b68e-f79338020327").unwrap() }),
        Box::new(TaskRenamed { name: format!("Renamed {:?}", time::get_time()), event_id: uuid::Uuid::parse_str("cbad187b-2fd0-4ad2-b78b-80d83f1ff303").unwrap() })
    ];

    let client = Client::new();
    let stream_name = test_stream_name();
    client.append_to_stream(&stream_name, ExpectedVersion::NotExist, events);
    let stream = client.read_stream_events_forward(&stream_name, 0, 1, true).unwrap();

    assert_eq!("task-renamed", stream.entries[0].event_type);
    assert_eq!("task-created", stream.entries[1].event_type);
    assert_eq!("cbad187b-2fd0-4ad2-b78b-80d83f1ff303", stream.entries[0].event_id);
    assert_eq!("baca1a30-b6f1-470b-b68e-f79338020327", stream.entries[1].event_id);
}

fn test_stream_name() -> String {
    format!("task-{}", time::get_time().sec)
}
