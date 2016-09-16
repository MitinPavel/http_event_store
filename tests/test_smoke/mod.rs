extern crate time;
extern crate http_event_store as es;


#[test]
fn it_interacts_with_event_store() {
    let client = es::client::Client::new();
    let stream_name = test_stream_name();

    let raw_json = r#"[
                     {
                       "eventId": "50ed34a2-b26e-4610-8a2e-35ae8e63599e",
                       "eventType": "task-created",
                       "data": {
                         "name": "Initial Name"
                       }
                     },
                     {
                       "eventId": "26e9fdd6-d7fe-4acf-b8dc-a494300963dc",
                       "eventType": "task-renamed",
                       "data": {
                         "name": "Updated Name"
                       }
                     }
                   ]"#;

    client.append_to_stream(&stream_name, 987, raw_json.into());

    client.read_stream_events_forward(&stream_name, 0, 1, true);

    println!("Hello")
}

fn test_stream_name() -> String {
 format!("task-{}", time::get_time().sec)
}
