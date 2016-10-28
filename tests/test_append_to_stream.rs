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
use hes::error::HesError::*;
use hes::error::UserErrorKind::*;

#[test]
fn it_appends_events_in_right_order() {
    let events: Vec<Event> = vec![
        TaskCreated { name: format!("Created {:?}", time::get_time()), event_id: uuid::Uuid::parse_str("baca1a30-b6f1-470b-b68e-f79338020327").unwrap() }.into(),
        TaskRenamed { name: format!("Renamed {:?}", time::get_time()), event_id: uuid::Uuid::parse_str("cbad187b-2fd0-4ad2-b78b-80d83f1ff303").unwrap() }.into()
    ];

    let client = Client::default();
    let stream_name = test_stream_name();
    client.append_to_stream(&stream_name, ExpectedVersion::NoStream, events).unwrap();
    let stream = client.read_stream_events_forward(&stream_name, 0, 2, true).unwrap();

    assert_eq!("task-renamed", stream.entries[0].event_type);
    assert_eq!("task-created", stream.entries[1].event_type);
    assert_eq!("cbad187b-2fd0-4ad2-b78b-80d83f1ff303", stream.entries[0].event_id);
    assert_eq!("baca1a30-b6f1-470b-b68e-f79338020327", stream.entries[1].event_id);
}

#[test]
fn it_requires_expected_version_to_be_correct() {
    let client = Client::default();
    let stream_name = test_stream_name();

    let mut version = ExpectedVersion::NoStream;
    client.append_to_stream(&stream_name, version, vec![task_created_event().into()]).unwrap();
    assert_eq!(1, client.read_stream_events_forward(&stream_name, 0, 3, true).unwrap().entries.len());

    version = ExpectedVersion::Number(0);
    client.append_to_stream(&stream_name, version, vec![task_renamed_event().into()]).unwrap();
    assert_eq!(2, client.read_stream_events_forward(&stream_name, 0, 3, true).unwrap().entries.len());

    version = ExpectedVersion::Number(1);
    client.append_to_stream(&stream_name, version, vec![task_renamed_event().into()]).unwrap();
    assert_eq!(3, client.read_stream_events_forward(&stream_name, 0, 3, true).unwrap().entries.len());

    println!("{:?}", client.read_stream_events_forward(&stream_name, 0, 3, true).unwrap().entries)
}

#[test]
fn it_returns_event_number_mismatch_error_if_expected_version_is_wrong() {
    let client = Client::default();
    let stream_name = test_stream_name();

    let version = ExpectedVersion::Number(1);
    let result = client.append_to_stream(&stream_name, version, vec![task_created_event().into()]);

    assert_error!(UserError, EventNumberMismatch(..), result.unwrap_err());
}

#[test]
fn it_returns_bad_request_error_if_event_data_is_malformed() {
    let client = Client::default();
    let stream_name = test_stream_name();

    let malformed_event = hes::event::Event {
        event_id: uuid::Uuid::new_v4(),
        event_type: "task-created".to_string(),
        data: Some("?-/*".to_string())
    };
    let result = client.append_to_stream(&stream_name, ExpectedVersion::NoStream, vec![malformed_event]);

    assert_error!(UserError, BadRequest(..), result.unwrap_err());
}


fn test_stream_name() -> String {
    format!("task-{}", uuid::Uuid::new_v4().simple())
}

fn task_created_event() -> TaskCreated {
    TaskCreated { name: format!("Created {:?}", time::get_time()), event_id: uuid::Uuid::new_v4() }
}
fn task_renamed_event() -> TaskRenamed {
    TaskRenamed { name: format!("Renamed {:?}", time::get_time()), event_id: uuid::Uuid::new_v4() }
}
