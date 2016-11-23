extern crate uuid;
extern crate serde_json;
extern crate http_event_store as hes;

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

impl From<TaskCreated> for hes::write::Event {
    fn from(e: TaskCreated) -> Self {
        hes::write::Event {
            event_id: e.event_id,
            event_type: "task-created".to_string(),
            data: Some(serde_json::from_str(&format!(r#"{{ "name": "{}" }}"#, e.name)).unwrap())
        }
    }
}

impl From<TaskRenamed> for hes::write::Event {
    fn from(e: TaskRenamed) -> Self {
        hes::write::Event {
            event_id: e.event_id,
            event_type: "task-renamed".to_string(),
            data: Some(serde_json::from_str(&format!(r#"{{ "name": "{}" }}"#, e.name)).unwrap())
        }
    }
}
