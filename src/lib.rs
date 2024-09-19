use anyhow::anyhow;

use anyhow::Result;
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
    Create,
    Delete,
    Get,
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

    pub fn submit_query(&self, method: Method, key: &str) -> Result<u8> {
        let query = Query { key: key.to_string(), method };
        let query_bytes = serde_json::to_vec(&query).expect("Failed to serialize query");
        let response = self.door.call_with_data(&query_bytes).expect("Door call failed");
        let response: Result<u8, String> =
            serde_json::from_slice(response.data()).expect("Failed to deserialize response");
        match response {
            Ok(value) => Ok(value),
            Err(context) => Err(anyhow!(context))
        }
    }
}
