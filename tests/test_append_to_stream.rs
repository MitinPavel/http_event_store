extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate time;
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
fn should_append_events_in_right_order() {
    let created_id = uuid::Uuid::new_v4();
    let updated_id = uuid::Uuid::new_v4();
    let events: Vec<Event> = vec![
        TaskCreated { name: format!("Created {:?}", time::get_time()), event_id: created_id }.into(),
        TaskRenamed { name: format!("Renamed {:?}", time::get_time()), event_id: updated_id }.into()
    ];

    let client = Client::default();
    let stream_name = test_stream_name();
    client.append_to_stream(&stream_name, ExpectedVersion::NoStream, events).unwrap();
    let stream = client.read_stream_events_forward(&stream_name, 0, 2, true).unwrap();

    assert_eq!("task-renamed", stream.entries[0].event_type);
    assert_eq!("task-created", stream.entries[1].event_type);
    assert_eq!(updated_id, stream.entries[0].event_id);
    assert_eq!(created_id, stream.entries[1].event_id);
    assert_eq!(1, stream.entries[0].event_number);
    assert_eq!(0, stream.entries[1].event_number);
}

#[test]
fn should_require_expected_version_to_be_correct() {
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
}

#[test]
fn should_return_wrong_expected_event_number_error_if_expected_version_is_wrong() {
    let client = Client::default();
    let stream_name = test_stream_name();

    let version = ExpectedVersion::Number(1);
    let result = client.append_to_stream(&stream_name, version, vec![task_created_event().into()]);

    assert_error!(WrongExpectedEventNumber(..), result.unwrap_err());
}

#[test]
fn should_fail_appending_with_any_expected_version_to_deleted_stream() {
    let client = Client::default();
    let stream_name = test_stream_name();

    client.append_to_stream(&stream_name, ExpectedVersion::NoStream, vec![task_created_event().into()]).unwrap();
    client.hard_delete_stream(&stream_name, ExpectedVersion::Any).unwrap();

    let result = client.append_to_stream(&stream_name, ExpectedVersion::Any, vec![task_renamed_event().into()]);

    assert_error!(StreamDeleted(..), result.unwrap_err());
}

#[test]
fn should_cope_with_empty_event_collection() {}

fn test_stream_name() -> String {
    format!("task-{}", uuid::Uuid::new_v4().simple())
}

fn task_created_event() -> TaskCreated {
    TaskCreated { name: format!("Created {:?}", time::get_time()), event_id: uuid::Uuid::new_v4() }
}
fn task_renamed_event() -> TaskRenamed {
    TaskRenamed { name: format!("Renamed {:?}", time::get_time()), event_id: uuid::Uuid::new_v4() }
}
