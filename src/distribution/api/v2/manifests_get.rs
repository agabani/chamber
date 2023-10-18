use std::{future::Future, pin::Pin};

use crate::distribution::{
    self,
    authentication::{Authentication, Credential},
    error,
};

/// application/vnd.docker.distribution.manifest.v1+json
pub const APPLICATION_VND_DOCKER_DISTRIBUTION_MANIFEST_V1_JSON: &'static str =
    "application/vnd.docker.distribution.manifest.v1+json";

/// application/vnd.docker.distribution.manifest.v1+prettyjws
pub const APPLICATION_VND_DOCKER_DISTRIBUTION_MANIFEST_V1_PRETTYJWS: &'static str =
    "application/vnd.docker.distribution.manifest.v1+prettyjws";

/// application/vnd.docker.distribution.manifest.v2+json
pub const APPLICATION_VND_DOCKER_DISTRIBUTION_MANIFEST_V2_JSON: &'static str =
    "application/vnd.docker.distribution.manifest.v2+json";

/// application/vnd.docker.distribution.manifest.list.v2+json
pub const APPLICATION_VND_DOCKER_DISTRIBUTION_MANIFEST_LIST_V2_JSON: &'static str =
    "application/vnd.docker.distribution.manifest.list.v2+json";

///
pub struct Request {
    _authentication: Option<Authentication>,
    _base_url: url::Url,
    _credential: Option<Credential>,
    _name: String,
    _reference: String,
    _accept: Vec<String>,
}

impl Request {
    ///
    #[must_use]
    pub fn new(
        base_url: url::Url,
        authentication: Option<Authentication>,
        credential: Option<Credential>,
        name: String,
        reference: String,
        accept: Vec<String>,
    ) -> Self {
        Self {
            _authentication: authentication,
            _base_url: base_url,
            _credential: credential,
            _name: name,
            _reference: reference,
            _accept: accept,
        }
    }
}

impl distribution::Request for Request {
    type Future = Pin<Box<dyn Future<Output = Result<hyper::Request<hyper::Body>, error::Error>>>>;

    fn authentication(&self) -> Option<&Authentication> {
        todo!()
    }

    fn credential(&self) -> Option<&Credential> {
        todo!()
    }

    fn to_http_request(&self, _authentication: Option<&Authentication>) -> Self::Future {
        todo!()
    }
}

///
pub struct Response {
    authentication: Option<Authentication>,
    http_response: hyper::Response<hyper::Body>,
}

impl Response {
    ///
    pub fn authentication(&self) -> Option<&Authentication> {
        self.authentication.as_ref()
    }

    ///
    pub fn raw(&self) -> &hyper::Response<hyper::Body> {
        &self.http_response
    }
}

impl distribution::Response for Response {
    type Future = Pin<Box<dyn Future<Output = Result<Self, error::Error>>>>;

    fn new(
        _http_response: hyper::Response<hyper::Body>,
        _authentication: Option<Authentication>,
    ) -> Self::Future {
        todo!()
    }

    fn www_authenticate(
        &self,
    ) -> Result<Option<distribution::www_authenticate::WwwAuthenticate>, error::Error> {
        todo!()
    }
}
