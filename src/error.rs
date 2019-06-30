use failure::Fail;
use std::io;

///
#[derive(Fail, Debug)]
pub enum KvsError {
    #[fail(display = "Serde error: {}", _0)]
    Serde(#[cause] serde_json::Error),
    #[fail(display = "Io error: {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "An unknown error has occurred.")]
    UnknownError,
}

//std::convert::From

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> Self {
        KvsError::Serde(err)
    }
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> Self {
        KvsError::Io(err)
    }
}

///
pub type Result<T> = std::result::Result<T, KvsError>;
