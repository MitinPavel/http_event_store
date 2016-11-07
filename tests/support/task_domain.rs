extern crate uuid;
extern crate http_event_store as es;

//#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
pub struct TaskCreated {
    pub event_id: uuid::Uuid,
    pub name: String
}

#[derive(Clone)]
pub struct TaskRenamed {
    pub event_id: uuid::Uuid,
    pub name: String
}

impl From<TaskCreated> for es::event::Event {
    fn from(e: TaskCreated) -> Self {
        es::event::Event {
            event_id: Some(e.event_id),
            event_type: "task-created".to_string(),
            data: Some(format!(r#"{{ "name": "{}" }}"#, e.name))
        }
    }
}

impl From<TaskRenamed> for es::event::Event {
    fn from(e: TaskRenamed) -> Self {
        es::event::Event {
            event_id: Some(e.event_id),
            event_type: "task-renamed".to_string(),
            data: Some(format!(r#"{{ "name": "{}" }}"#, e.name))
        }
    }
}
