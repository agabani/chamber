use std::{future::Future, pin::Pin};

use crate::distribution::{
    self,
    authentication::{Authentication, Credential},
    error,
    www_authenticate::WwwAuthenticate,
};

///
pub struct Request {
    authentication: Option<Authentication>,
    base_url: url::Url,
    credential: Option<Credential>,
    name: String,
    digest: String,
}

impl Request {
    ///
    #[must_use]
    pub fn new(
        base_url: url::Url,
        authentication: Option<Authentication>,
        credential: Option<Credential>,
        name: String,
        digest: String,
    ) -> Self {
        Self {
            authentication,
            base_url,
            credential,
            name,
            digest,
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
        base_url.set_path(&format!("/v2/{}/blobs/{}", self.name, self.digest));

        let uri = base_url.to_string();

        let mut request = hyper::Request::builder()
            .method(hyper::Method::GET)
            .uri(uri);

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
