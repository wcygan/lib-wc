#[derive(Debug)]
pub enum Error {
    Ignored,
    Message(String),
    Io(std::io::Error),
}

impl From<&str> for Error {
    fn from(src: &str) -> Error {
        Error::Message(src.into())
    }
}

impl From<String> for Error {
    fn from(src: String) -> Error {
        Error::Message(src)
    }
}

impl From<std::io::Error> for Error {
    fn from(src: std::io::Error) -> Error {
        Error::Io(src)
    }
}
