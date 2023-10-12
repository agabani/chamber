#![deny(clippy::pedantic, missing_docs)]
#![forbid(unsafe_code)]

//! Crate for interacting with the container ecosystem.

/// Module for interacting with the container distribution ecosystem.
pub mod distribution;

/// Possible errors when working with [`chamber`][crate].
#[derive(Debug)]
pub enum Error {
    /// Hyper error.
    Hyper(hyper::Error),
}

/// Convenient alias for [`std::result::Result<T, crate::Error>`].
pub type Result<T, E = crate::Error> = std::result::Result<T, E>;
