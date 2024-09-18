use serde_derive::{Deserialize, Serialize};
use serde_json;
use doors;

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub key: String,
    pub method: Method,
}

#[derive(Serialize, Deserialize)]
pub enum Method {
    Get,
    Create,
    Delete,
    Increment,
}

pub static PATH: &str = "/tmp/doordb";

pub struct Client {
    door: doors::Client,
}

impl Client {
    pub fn new() -> Self {
        let door = doors::Client::open(PATH).expect("Failed to open door");
        Self{ door }
    }

    pub fn submit_query(&self, method: Method, key: String) -> Result<u8, String> {
        let query = Query { key, method };
        let query_bytes = serde_json::to_vec(&query).expect("Failed to serialize query");
        let response = self.door.call_with_data(&query_bytes).expect("Door call failed");
        let response: Result<u8, String> =
            serde_json::from_slice(response.data()).expect("Failed to deserialize response");
        response
    }
}
