use std::io;

#[derive(Debug)]
pub enum SquidError {
    Io(io::Error),
    InvalidHeader(String),
    UnsupportedFormat(String),
    InvalidData(String),
}

impl From<io::Error> for SquidError {
    fn from(err: io::Error) -> Self {
        SquidError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, SquidError>;
