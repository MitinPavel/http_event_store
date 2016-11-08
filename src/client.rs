use api::append_to_stream::Appender;
use api::read_stream_events_forward::Reader;
use api::delete_stream::Deleter;
use Stream;
use event::Event;
use types::Result;
use expected_version::ExpectedVersion;
use connection::ConnectionInfo;

pub struct Client {
    connection_info: ConnectionInfo,
}

impl Client {
    pub fn new(connection_info: ConnectionInfo) -> Client {
        Client { connection_info: connection_info }
    }

    pub fn default() -> Client {
        Client {
            connection_info: ConnectionInfo {
                host: "127.0.0.1".into(),
                port: 2113
            }
        }
    }

    pub fn append_to_stream<I>(&self,
                               stream_name: &str,
                               expected_version: ExpectedVersion,
                               events: I)
                               -> Result<()>
        where I: IntoIterator<Item = Event>
    {
        let appender = Appender::new(&self.connection_info);
        appender.append(stream_name, expected_version, events)
    }

    //TODO Restrict `count` using u8 or u16
    pub fn read_stream_events_forward(&self, stream_name: &str, start: u32, count: u32, resolve_link_tos: bool) -> Result<Stream> {
        let reader = Reader {};
        reader.read_stream_events_forward(stream_name, start, count, resolve_link_tos)
    }

    pub fn delete_stream(&self, stream_name: &str, expected_version: ExpectedVersion) -> Result<()> {
        let deleter = Deleter::new(&self.connection_info);
        deleter.delete(stream_name, expected_version)
    }

    pub fn hard_delete_stream(&self, stream_name: &str, expected_version: ExpectedVersion) -> Result<()> {
        let deleter = Deleter::new(&self.connection_info);
        deleter.hard_delete(stream_name, expected_version)
    }
}
