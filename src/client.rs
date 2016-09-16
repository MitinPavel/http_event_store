use api::Api;
use Stream;

pub struct Client {
    api: Api
}

impl Client {
    pub fn new() -> Client {
        Client { api: Api {} }
    }

    pub fn append_to_stream(&self, stream_name: &str, expectedVersion: u64, events: String) {
        self.api.append_to_stream(stream_name, expectedVersion, events)
    }

    // TODO Restrict `count` using u8 or u16
    pub fn read_stream_events_forward(&self, stream_name: &str, start: u32, count: u32, resolve_link_tos: bool) -> Result<Stream, String> {
        self.api.read_stream_events_forward(stream_name, start, count, resolve_link_tos)
    }

    pub fn get(&self) {
        self.api.get()
    }
}
