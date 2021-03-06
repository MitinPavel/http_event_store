use hyper::Client as HyperClient;
use serde;

use api::append_to_stream::Appender;
use api::read_stream_events_forward::StreamReader;
use api::delete_stream::Deleter;
use read::Stream;
use read::EmbedLevel;
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

    pub fn read_stream_events_forward<E: serde::Deserialize + EmbedLevel>(&self,
                                      stream_name: &str,
                                      start: i32,
                                      count: i32,
                                      resolve_link_tos: bool)
                                      -> Result<Stream<E>, ApiError> {
        let reader = StreamReader::new(&self.connection_info, &self.http_client);
        reader.read_forward::<E>(stream_name, start, count, resolve_link_tos)
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
