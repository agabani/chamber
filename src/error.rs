use std::{error, fmt};

/// Alias for a type-erased error type.
#[allow(clippy::module_name_repetitions)]
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// Possible errors when working with [`chamber`][crate].
#[derive(Debug)]
pub enum Error {
    /// Box error.
    Box(BoxError),

    /// Hyper error.
    Hyper(hyper::Error),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<BoxError> for Error {
    fn from(value: BoxError) -> Self {
        match value.downcast::<hyper::Error>() {
            Ok(err) => Error::Hyper(*err),
            Err(err) => Error::Box(err),
        }
    }
}
