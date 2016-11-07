use uuid;

pub struct Event {
    pub event_id: Option<uuid::Uuid>,
    pub event_type: String,
    pub data: Option<String>
}
