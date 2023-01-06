pub mod connection;
pub mod requests;
pub static DEFAULT_ADDRESS: &str = "localhost:7272";

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TracingInitializationError,
    Ignored,
    Message(String),
    IoError(std::io::Error),
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error::Message(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}
