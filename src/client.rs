use api::Api;
use Stream;
use event::Event;
use types::Result;
use expected_version::ExpectedVersion;

pub struct Client {
    api: Api
}

impl Client {
    pub fn new() -> Client {
        Client { api: Api {} }
    }

    pub fn append_to_stream(&self, stream_name: &str, expected_version: ExpectedVersion, events: Vec<Event>) -> Result<()> {
        self.api.append_to_stream(stream_name, expected_version, events)
    }

    // TODO Restrict `count` using u8 or u16
    pub fn read_stream_events_forward(&self, stream_name: &str, start: u32, count: u32, resolve_link_tos: bool) -> Result<Stream> {
        self.api.read_stream_events_forward(stream_name, start, count, resolve_link_tos)
    }
}
