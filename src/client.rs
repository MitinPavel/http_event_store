use api::append_to_stream::Appender;
use api::read_stream_events_forward::Reader;
use Stream;
use event::Event;
use types::Result;
use expected_version::ExpectedVersion;

pub struct Client {
    //host: &str
    //port: ...
}

impl Client {
    pub fn new() -> Client {
        Client {}
    }

    pub fn append_to_stream(&self, stream_name: &str, expected_version: ExpectedVersion, events: Vec<Event>) -> Result<()> {
        let appender = Appender {};
        appender.append_to_stream(stream_name, expected_version, events)
    }

    //TODO Restrict `count` using u8 or u16
    pub fn read_stream_events_forward(&self, stream_name: &str, start: u32, count: u32, resolve_link_tos: bool) -> Result<Stream> {
        let reader = Reader {};
        reader.read_stream_events_forward(stream_name, start, count, resolve_link_tos)
    }
}
