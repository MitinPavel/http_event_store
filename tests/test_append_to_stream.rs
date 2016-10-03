extern crate serde;
extern crate serde_json;
extern crate time;
extern crate uuid;
extern crate http_event_store as es;

mod support;

use support::*;

#[test]
fn it_interacts_with_event_store() {
    let client = es::client::Client::new();
    let stream_name = test_stream_name();
    let created_event_id =  uuid::Uuid::new_v4();

    let mut events: Vec<Box<es::event::Event>> = vec![
        Box::new(TaskCreated { name: format!("Created {:?}", time::get_time()), event_id: created_event_id.clone() }),
        Box::new(TaskRenamed { name: format!("Renamed {:?}", time::get_time()), event_id: uuid::Uuid::new_v4() })
    ];

    client.append_to_stream(&stream_name, 987, events);

    let stream = client.read_stream_events_forward(&stream_name, 0, 1, true).unwrap();

    assert_eq!("task-renamed", stream.entries[0].event_type);
    assert_eq!("task-created", stream.entries[1].event_type);
    assert_eq!(created_event_id.hyphenated().to_string(), stream.entries[1].event_id);
}

fn test_stream_name() -> String {
    format!("task-{}", time::get_time().sec)
}
