use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub action: Action,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    /// Increase the ping count
    Ping,
    /// Increase the pong count
    Pong,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    /// The action taken
    pub action: Action,
    /// The total number of times that the action has been taken (inclusive)
    pub count: u32,
}

impl Request {
    pub fn ping() -> Self {
        Self {
            action: Action::Ping,
        }
    }

    pub fn pong() -> Self {
        Self {
            action: Action::Pong,
        }
    }
}

impl Action {
    pub fn response(self, count: u32) -> Response {
        Response {
            action: self,
            count,
        }
    }
}
