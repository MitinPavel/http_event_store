use hyper::Client as HyperClient;
use serde;

use api::append_to_stream::Appender;
use api::read_stream_events_forward::Reader;
use api::delete_stream::Deleter;
use read::Stream;
use write::Event;
use error::ApiError;
use expected_version::ExpectedVersion;
use connection::ConnectionInfo;

pub struct Client {
    connection_info: ConnectionInfo,
    http_client: HyperClient,
}

impl Client {
    pub fn new(connection_info: ConnectionInfo) -> Client {
        Client { connection_info: connection_info, http_client: HyperClient::default() }
    }

    pub fn default() -> Client {
        Client {
            connection_info: ConnectionInfo {
                host: "127.0.0.1".into(),
                port: 2113
            },
            http_client: HyperClient::default()
        }
    }

    pub fn append_to_stream<I>(&self,
                               stream_name: &str,
                               expected_version: ExpectedVersion,
                               events: I)
                               -> Result<(), ApiError>
        where I: IntoIterator<Item = Event> {
        let appender = Appender::new(&self.connection_info, &self.http_client);
        appender.append(stream_name, expected_version, events)
    }

    //TODO Restrict `count` using u8 or u16
    pub fn read_stream_events_forward<E: serde::Deserialize>(&self,
                                      stream_name: &str,
                                      start: u32,
                                      count: u32,
                                      resolve_link_tos: bool)
                                      -> Result<Stream<E>, ApiError> {
        let reader = Reader::new(&self.connection_info, &self.http_client);
        reader.read_stream_events_forward::<E>(stream_name, start, count, resolve_link_tos)
    }

    pub fn delete_stream(&self,
                         stream_name: &str,
                         expected_version: ExpectedVersion)
                         -> Result<(), ApiError> {
        let deleter = Deleter::new(&self.connection_info, &self.http_client);
        deleter.delete(stream_name, expected_version)
    }

    pub fn hard_delete_stream(&self,
                              stream_name: &str,
                              expected_version: ExpectedVersion)
                              -> Result<(), ApiError> {
        let deleter = Deleter::new(&self.connection_info, &self.http_client);
        deleter.hard_delete(stream_name, expected_version)
    }
}
