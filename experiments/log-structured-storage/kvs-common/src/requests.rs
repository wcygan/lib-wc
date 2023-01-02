use actix::Message;
use serde_derive::{Deserialize, Serialize};

// todo: fill this enum with the right types
#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub enum Request {
    Put,
    Get,
    Update,
    Delete,
}

// todo: fill this struct in with the right types (some form of byte slice &[u8]?)
#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct Response {
    key: String,
    value: String,
}
