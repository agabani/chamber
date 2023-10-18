use std::{future::Future, pin::Pin};

use crate::{
    distribution::{
        authentication::{Authentication, Credential},
        error,
        service::{Request, Response},
    },
    parser::www_authenticate::WwwAuthenticate,
};

///
pub struct SupportRequest {
    authentication: Option<Authentication>,
    base_url: url::Url,
    credential: Option<Credential>,
}

impl SupportRequest {
    ///
    #[must_use]
    pub fn new(
        base_url: url::Url,
        authentication: Option<Authentication>,
        credential: Option<Credential>,
    ) -> Self {
        Self {
            authentication,
            base_url,
            credential,
        }
    }
}

impl Request for SupportRequest {
    type Future = Pin<Box<dyn Future<Output = Result<hyper::Request<hyper::Body>, error::Error>>>>;

    fn authentication(&self) -> Option<&Authentication> {
        self.authentication.as_ref()
    }

    fn credential(&self) -> Option<&Credential> {
        self.credential.as_ref()
    }

    fn to_http_request(&self, authentication: Option<&Authentication>) -> Self::Future {
        let mut base_url = self.base_url.clone();
        base_url.set_path("/v2/");

        let uri = base_url.to_string();

        let mut request = hyper::Request::builder()
            .method(hyper::Method::GET)
            .uri(uri);

        if let Some(authentication) = authentication.or(self.authentication.as_ref()) {
            match authentication {
                Authentication::Basic(token) => {
                    request = request.header("Authorization", format!("Basic {token}"));
                }
                Authentication::Bearer(_) => todo!(),
            }
        }

        let result = request.body(hyper::body::Body::empty());

        Box::pin(async move { result.map_err(Into::into) })
    }
}

///
pub struct SupportResponse {
    authentication: Option<Authentication>,
    http_response: hyper::Response<hyper::Body>,
}

impl SupportResponse {
    ///
    pub fn authentication(&self) -> Option<&Authentication> {
        self.authentication.as_ref()
    }

    ///
    pub fn raw(&self) -> &hyper::Response<hyper::Body> {
        &self.http_response
    }
}

impl Response for SupportResponse {
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

    fn www_authenticate(&self) -> Result<Option<WwwAuthenticate>, error::Error> {
        let Some(header) = self.http_response.headers().get("Www-Authenticate") else {
            return Ok(None);
        };
        let value = header.to_str()?;
        let www_authenticate = WwwAuthenticate::parse(value)?;
        Ok(Some(www_authenticate))
    }
}
