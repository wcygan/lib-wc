pub mod connection;
pub mod error;
pub mod messages;

pub type Result<T> = std::result::Result<T, error::Error>;

/// The address that the client and server will operate on
pub static ADDRESS: &str = "localhost:7272";
