use api::Api;

pub struct Client {
    api: Api
}

impl Client {
    pub fn new() -> Client {
        Client { api: Api {} }
    }

    pub fn get(&self) {
        self.api.get()
    }
}
