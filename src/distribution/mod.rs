///
pub mod api;
///
pub mod authentication;
///
pub mod error;
///
pub mod service;
///
pub mod spec;
///
pub mod streaming;
///
pub mod www_authenticate;

use std::future::Future;

use self::{
    authentication::{Authentication, Credential},
    www_authenticate::WwwAuthenticate,
};

///
pub trait Request {
    ///
    type Future: Future<Output = Result<hyper::Request<hyper::Body>, error::Error>>;

    ///
    fn authentication(&self) -> Option<&Authentication>;

    ///
    fn credential(&self) -> Option<&Credential>;

    ///
    fn to_http_request(&self, authentication: Option<&Authentication>) -> Self::Future;
}

///
pub trait Response
where
    Self: Sized,
{
    ///
    type Future: Future<Output = Result<Self, error::Error>>;

    ///
    fn new(
        http_response: hyper::Response<hyper::Body>,
        authentication: Option<Authentication>,
    ) -> Self::Future;

    /// # Errors
    ///
    /// Will return `Err` if Www-Authenticate header is unparsable.
    fn www_authenticate(&self) -> Result<Option<WwwAuthenticate>, error::Error>;
}
