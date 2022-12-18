use serde_derive::{Deserialize, Serialize};

/// The [`PingPong`] protocol is used to bounce N messages between a client and server
///
/// When a server/client recieves a [`PingPong`],
/// it should respond by [`PingPong::bounce`]ing the message back to the sender.
#[derive(Debug, Serialize, Deserialize)]
pub struct PingPongPacket {
    pub pingpong: Option<PingPong>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PingPong {
    Ping(u8),
    Pong(u8),
}

impl PingPong {
    /// Return the next [`PingPong`] message to send.
    /// A [`PingPong`] will bounce until its counter reaches 0, and then None is returned.
    pub fn bounce(&self) -> Option<PingPong> {
        match self {
            PingPong::Ping(n) => match n {
                0 => None,
                _ => Some(PingPong::Pong(*n - 1)),
            },
            PingPong::Pong(n) => match n {
                0 => None,
                _ => Some(PingPong::Ping(*n - 1)),
            },
        }
    }
}
