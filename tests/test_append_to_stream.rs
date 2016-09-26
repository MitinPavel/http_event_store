extern crate serde;
extern crate serde_json;
extern crate time;
extern crate uuid;
extern crate http_event_store as es;

//#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
struct TaskCreated {
    event_id: uuid::Uuid,
    name: String
}

impl TaskCreated {
    pub fn to_string(&self) -> String {
        format!(r#"{{"eventType": "task-created",
                     "eventId": "{}",
                     "data": {{
                       "name": "{}"
                     }}
                   }}"#,
                self.event_id.hyphenated().to_string(),
                self.name)
    }
}

impl serde::Serialize for TaskCreated {
    fn serialize<S: serde::Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[test]
fn it_interacts_with_event_store() {
    let client = es::client::Client::new();
    let stream_name = test_stream_name();

    let events = vec![TaskCreated { name: "A new task 09:31".to_string(), event_id: uuid::Uuid::new_v4() },
                      TaskCreated { name: "A new task 09:32".to_string(), event_id: uuid::Uuid::new_v4() }];

    let events_as_json : Vec<String> = events.iter().map(|e| serde_json::to_string(&e).unwrap()).collect::<Vec<String>>();
    let events_json : String = format!("[{}]", events_as_json.join(","));

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

    let stream = client.read_stream_events_forward(&stream_name, 0, 1, true).unwrap();

    println!("{:?}", stream)
}

fn test_stream_name() -> String {
    format!("task-{}", time::get_time().sec)
}
