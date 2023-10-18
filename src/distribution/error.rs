use std::{error, fmt};

///
#[derive(Debug)]
pub enum Error {
    /// Hyper.
    Hyper(hyper::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl error::Error for Error {}

impl From<hyper::Error> for Error {
    fn from(value: hyper::Error) -> Self {
        Self::Hyper(value)
    }
}
