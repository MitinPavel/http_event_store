extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate http_event_store as hes;

#[macro_use]
mod support;

use support::task_domain::*;

use hes::write::Event;
use hes::client::Client;
use hes::expected_version::ExpectedVersion;
use hes::error::ApiError::*;

#[test]
fn should_hard_delete_stream() {
    let events: Vec<Event> = vec![TaskCreated { name: "Created".to_string(), event_id: uuid::Uuid::new_v4() }.into()];

    let client = Client::default();
    let stream_name = test_stream_name();

    client.append_to_stream(&stream_name, ExpectedVersion::NoStream, events).unwrap();
    assert!(client.read_stream_events_forward(&stream_name, 0, 1, true).is_ok());
    assert!(client.hard_delete_stream(&stream_name, ExpectedVersion::Any).is_ok());
    let result = client.read_stream_events_forward(&stream_name, 0, 1, true);
    assert_error!(StreamDeleted(..), result.unwrap_err());
}

#[test]
fn should_fail_if_expected_version_is_not_correct() {
    let events: Vec<Event> = vec![TaskCreated { name: "Created".to_string(), event_id: uuid::Uuid::new_v4() }.into()];

    let client = Client::default();
    let stream_name = test_stream_name();

    client.append_to_stream(&stream_name, ExpectedVersion::NoStream, events).unwrap();
    let result = client.hard_delete_stream(&stream_name, ExpectedVersion::NoStream);
    assert_error!(WrongExpectedEventNumber(..), result.unwrap_err());
}

//TODO Turn into a function on support::task_domain.
fn test_stream_name() -> String {
    format!("task-{}", uuid::Uuid::new_v4().simple())
}
