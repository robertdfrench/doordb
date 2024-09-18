use serde_derive::{Deserialize, Serialize};

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
