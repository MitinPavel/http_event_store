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
