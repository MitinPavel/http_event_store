use uuid;

pub struct Event {
    pub event_id: uuid::Uuid,
    pub event_type: String,
    pub data: Option<String>
}
