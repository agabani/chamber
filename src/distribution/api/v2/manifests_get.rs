use std::{future::Future, pin::Pin};

use crate::distribution::{
    self,
    authentication::{Authentication, Credential},
    error,
    www_authenticate::WwwAuthenticate,
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
    authentication: Option<Authentication>,
    base_url: url::Url,
    credential: Option<Credential>,
    name: String,
    reference: String,
    accept: Vec<String>,
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
            authentication,
            base_url,
            credential,
            name,
            reference,
            accept,
        }
    }
}

impl distribution::Request for Request {
    type Future = Pin<Box<dyn Future<Output = Result<hyper::Request<hyper::Body>, error::Error>>>>;

    fn authentication(&self) -> Option<&Authentication> {
        self.authentication.as_ref()
    }

    fn credential(&self) -> Option<&Credential> {
        self.credential.as_ref()
    }

    fn to_http_request(&self, authentication: Option<&Authentication>) -> Self::Future {
        let mut base_url = self.base_url.clone();
        base_url.set_path(&format!("/v2/{}/manifests/{}", self.name, self.reference));

        let uri = base_url.to_string();

        let mut request = hyper::Request::builder()
            .method(hyper::Method::GET)
            .uri(uri);

        if !self.accept.is_empty() {
            let accept = self.accept.join(",");
            request = request.header("Accept", accept);
        }

        if let Some(authentication) = authentication.or(self.authentication.as_ref()) {
            match authentication {
                Authentication::Basic(token) => {
                    request = request.header("Authorization", format!("Basic {token}"));
                }
                Authentication::Bearer(bearer) => {
                    request =
                        request.header("Authorization", format!("Bearer {}", bearer.access_token));
                }
            }
        }

        let result = request.body(hyper::body::Body::empty());

        Box::pin(async move { result.map_err(Into::into) })
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
        http_response: hyper::Response<hyper::Body>,
        authentication: Option<Authentication>,
    ) -> Self::Future {
        let result = Ok(Self {
            authentication,
            http_response,
        });
        Box::pin(async move { result })
    }

    fn www_authenticate(
        &self,
    ) -> Result<Option<distribution::www_authenticate::WwwAuthenticate>, error::Error> {
        let Some(header) = self.http_response.headers().get("Www-Authenticate") else {
            return Ok(None);
        };
        let value = header.to_str()?;
        let www_authenticate = WwwAuthenticate::parse(value)?;
        Ok(Some(www_authenticate))
    }
}
