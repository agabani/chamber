#![deny(clippy::pedantic, missing_docs)]
#![forbid(unsafe_code)]

//! Crate for interacting with the container ecosystem.

///
pub mod distribution;

use std::future::Future;

///
pub trait Service<Request> {
    ///
    type Response;

    ///
    type Error;

    ///
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    ///
    fn call(&self, request: Request) -> Self::Future;
}
