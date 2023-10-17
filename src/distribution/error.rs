use std::{error, fmt};

///
#[derive(Debug)]
pub enum Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl error::Error for Error {}
