extern crate chrono;
extern crate http_event_store as hes;
extern crate uuid;

use hes::client::Client;
use hes::write::Event;
use hes::read::BodyEntry;
use hes::read::NoneEntry;
use hes::error::ApiError::*;
use hes::expected_version::ExpectedVersion;

#[macro_use]
mod support;

use support::task_domain::*;

#[test]
fn should_work_with_body_embed_entry_type() {
    let event: Event = TaskCreated {
        name: format!("Created {:?}", chrono::UTC::now()),
        event_id: uuid::Uuid::new_v4()
    }.into();

    let client = Client::default();
    let stream_name = test_stream_name();
    client.append_to_stream(&stream_name, ExpectedVersion::NoStream, vec![event]).unwrap();

    let entries: Vec<BodyEntry> = client.read_stream_events_forward::<BodyEntry>(&stream_name, 0, 1, true)
        .unwrap()
        .entries;
    assert_eq!("task-created", entries[0].summary);
}

#[test]
fn should_work_with_none_embed_entry_type() {
       let event: Event = TaskCreated {
        name: format!("Created {:?}", chrono::UTC::now()),
        event_id: uuid::Uuid::new_v4()
    }.into();

    let client = Client::default();
    let stream_name = test_stream_name();
    client.append_to_stream(&stream_name, ExpectedVersion::NoStream, vec![event]).unwrap();


    let entries: Vec<NoneEntry> = client.read_stream_events_forward::<NoneEntry>(&stream_name, 0, 1, true)
        .unwrap()
        .entries;
    assert_eq!("task-created", entries[0].summary);
}

#[test]
fn should_return_stream_not_found_error_attempting_to_read_nonexistent_stream() {
    let client = Client::default();
    let nonexistent_stream_name = "some-nonexistent";
    let result = client.read_stream_events_forward::<BodyEntry>(&nonexistent_stream_name, 0, 1, true);

    assert_error!(StreamNotFound(..), result.unwrap_err());
}

#[test]
fn should_return_stream_deleted_error_attempting_to_read_deleted_stream() {
    let events: Vec<Event> = vec![Event { event_id: uuid::Uuid::new_v4(),
                                          event_type: "created".to_string(),
                                          data: None }];

    let client = Client::default();
    let stream_name = format!("stream-{}", uuid::Uuid::new_v4().simple());
    client.append_to_stream(&stream_name, ExpectedVersion::NoStream, events).unwrap();

    assert!(client.read_stream_events_forward::<BodyEntry>(&stream_name, 0, 1, true).is_ok());
    assert!(client.hard_delete_stream(&stream_name, ExpectedVersion::Any).is_ok());

    let result = client.read_stream_events_forward::<BodyEntry>(&stream_name, 0, 1, true);
    assert_error!(StreamDeleted(..), result.unwrap_err());
}

fn test_stream_name() -> String {
    format!("task-{}", uuid::Uuid::new_v4().simple())
}
