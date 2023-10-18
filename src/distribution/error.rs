use std::{error, fmt};

///
#[derive(Debug)]
pub enum Error {
    /// Header.
    Header(hyper::header::ToStrError),
    /// Http.
    Http(hyper::http::Error),
    /// Hyper.
    Hyper(hyper::Error),
    /// Nom.
    Nom(String),
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

impl From<hyper::http::Error> for Error {
    fn from(value: hyper::http::Error) -> Self {
        Self::Http(value)
    }
}

impl From<hyper::header::ToStrError> for Error {
    fn from(value: hyper::header::ToStrError) -> Self {
        Self::Header(value)
    }
}

impl<'a> From<nom::Err<nom::error::Error<&'a str>>> for Error {
    fn from(value: nom::Err<nom::error::Error<&'a str>>) -> Self {
        Self::Nom(value.to_string())
    }
}
