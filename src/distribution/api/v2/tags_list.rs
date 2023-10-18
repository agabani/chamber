use std::{future::Future, pin::Pin};

use hyper::StatusCode;

use crate::distribution::{
    self,
    authentication::{Authentication, Credential},
    error, spec,
    www_authenticate::WwwAuthenticate,
};

///
pub struct Request {
    authentication: Option<Authentication>,
    base_url: url::Url,
    credential: Option<Credential>,
    name: String,
}

impl Request {
    ///
    #[must_use]
    pub fn new(
        base_url: url::Url,
        name: String,
        authentication: Option<Authentication>,
        credential: Option<Credential>,
    ) -> Self {
        Self {
            authentication,
            base_url,
            credential,
            name,
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
        base_url.set_path(&format!("/v2/{}/tags/list", self.name));

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
    /// # Errors
    ///
    /// Will return `Err` if response body is not deserializable.
    pub async fn to_spec(self) -> Result<ResponseBody, error::Error> {
        let status_code = self.http_response.status();
        let body = self.http_response.into_body();
        let bytes = hyper::body::to_bytes(body).await?;

        let response = match status_code {
            StatusCode::OK => ResponseBody::Ok(serde_json::from_slice(&bytes)?),
            StatusCode::NOT_FOUND => ResponseBody::Error(serde_json::from_slice(&bytes)?),
            status_code => todo!("{status_code}"),
        };
        Ok(response)
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

    fn www_authenticate(&self) -> Result<Option<WwwAuthenticate>, error::Error> {
        let Some(header) = self.http_response.headers().get("Www-Authenticate") else {
            return Ok(None);
        };
        let value = header.to_str()?;
        let www_authenticate = WwwAuthenticate::parse(value)?;
        Ok(Some(www_authenticate))
    }
}

///
#[derive(Debug)]
pub enum ResponseBody {
    ///
    Ok(spec::v2::TagsListResponseBody),
    ///
    Error(spec::v2::ErrorResponseBody),
}
