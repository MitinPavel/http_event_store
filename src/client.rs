use api::Api;

pub struct Client {
    api: Api
}

impl Client {
    pub fn new() -> Client {
        Client { api: Api {} }
    }

    pub fn append_to_stream(&self, stream_name: String, expectedVersion: u64, events: String) {
        self.api.append_to_stream(stream_name, expectedVersion, events)
    }

    pub fn get(&self) {
        self.api.get()
    }
}
