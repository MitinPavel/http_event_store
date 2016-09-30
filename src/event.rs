use uuid;

pub trait Event {
    fn event_id(&self) -> uuid::Uuid;
    fn event_type(&self) -> &str;
    fn data(&self) -> Option<String>;
}
