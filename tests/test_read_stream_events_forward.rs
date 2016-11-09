extern crate time;
extern crate http_event_store as hes;
extern crate uuid;

use hes::event::Event;
use hes::error::HesError::*;
use hes::error::UserErrorKind::*;
use hes::expected_version::ExpectedVersion;

#[macro_use]
mod support;

#[test]
fn should_retrun_stream_not_found_error_attempting_to_read_nonexistent_stream() {
    let client = hes::client::Client::default();
    let nonexistent_stream_name = "some-nonexistent";
    let result = client.read_stream_events_forward(&nonexistent_stream_name, 0, 1, true);

    assert_error!(UserError, StreamNotFound, result.unwrap_err());
}

#[test]
fn should_retrun_stream_deleted_error_attempting_to_read_deleted_stream() {
    let events: Vec<Event> = vec![Event { event_id: Some(uuid::Uuid::new_v4()),
                                          event_type: "created".to_string(),
                                          data: Some("{a:1}".to_string()) }];

    let client = hes::client::Client::default();
    let stream_name = format!("stream-{}", uuid::Uuid::new_v4().simple());
    client.append_to_stream(&stream_name, ExpectedVersion::NoStream, events).unwrap();

    assert!(client.read_stream_events_forward(&stream_name, 0, 1, true).is_ok());
    assert!(client.hard_delete_stream(&stream_name, ExpectedVersion::Any).is_ok());

    let result = client.read_stream_events_forward(&stream_name, 0, 1, true);
    assert_error!(UserError, StreamDeleted, result.unwrap_err());
}
