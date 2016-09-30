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

#[derive(Clone)]
struct TaskRenamed {
    event_id: uuid::Uuid,
    name: String
}

impl es::event::Event for TaskCreated {
    fn event_id(&self) -> uuid::Uuid { self.event_id }
    fn event_type(&self) -> &str { "task-created" }
    fn data(&self) -> Option<String> { Some(format!(r#"{{ "name": "{}" }}"#, self.name)) }
}

impl es::event::Event for TaskRenamed {
    fn event_id(&self) -> uuid::Uuid { self.event_id }
    fn event_type(&self) -> &str { "task-renamed" }
    fn data(&self) -> Option<String> { Some(format!(r#"{{ "name": "{}" }}"#, self.name)) }
}

#[test]
fn it_interacts_with_event_store() {
    let client = es::client::Client::new();
    let stream_name = test_stream_name();

    let mut events: Vec<Box<es::event::Event>> =  vec![Box::new(TaskCreated { name: format!("Created {:?}", time::get_time()), event_id: uuid::Uuid::new_v4() }),
                                                       Box::new(TaskRenamed { name: format!("Renamed {:?}", time::get_time()), event_id: uuid::Uuid::new_v4() })];


    client.append_to_stream(&stream_name, 987, events);

    let stream = client.read_stream_events_forward(&stream_name, 0, 1, true).unwrap();

    println!("{:?}", stream)
}

fn test_stream_name() -> String {
    format!("task-{}", time::get_time().sec)
}
