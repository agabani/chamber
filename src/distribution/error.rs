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
    /// Json.
    Json(serde_json::Error),
    /// Nom.
    Nom(String),
    /// Protocol.
    Protocol(String),
    /// Url.
    Url(url::ParseError),
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

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(value: nom::Err<nom::error::Error<&str>>) -> Self {
        Self::Nom(value.to_string())
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::Url(value)
    }
}
