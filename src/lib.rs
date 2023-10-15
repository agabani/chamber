#![deny(clippy::pedantic, missing_docs)]
#![forbid(unsafe_code)]

//! Crate for interacting with the container ecosystem.

/// Module for interacting with the container distribution ecosystem.
pub mod distribution;
mod error;
///
pub mod parser;

pub use error::{BoxError, Error};

/// Convenient alias for [`std::result::Result<T, Error>`].
pub type Result<T, E = Error> = std::result::Result<T, E>;
