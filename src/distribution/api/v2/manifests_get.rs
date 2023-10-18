use std::{future::Future, pin::Pin};

use hyper::StatusCode;

use crate::distribution::{
    self,
    authentication::{Authentication, Credential},
    error, spec,
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

    /// # Errors
    ///
    /// Will return `Err` if response body is not deserializable.
    pub async fn to_spec(self) -> Result<ResponseBody, error::Error> {
        let status_code = self.http_response.status();

        match status_code {
            StatusCode::OK => (),
            StatusCode::NOT_FOUND => {
                let body = self.http_response.into_body();
                let bytes = hyper::body::to_bytes(body).await?;
                return Ok(ResponseBody::Error(serde_json::from_slice(&bytes)?));
            }
            status_code => todo!("{status_code}"),
        };

        let Some(content_type) = self.http_response.headers().get("Content-Type") else {
            return Err(error::Error::Protocol(
                "response header did not contain Content-Type.".to_string(),
            ));
        };

        Ok(match content_type.to_str()? {
            APPLICATION_VND_DOCKER_DISTRIBUTION_MANIFEST_V1_JSON => {
                let body = self.http_response.into_body();
                let bytes = hyper::body::to_bytes(body).await?;
                ResponseBody::V1(serde_json::from_slice(&bytes)?)
            }
            APPLICATION_VND_DOCKER_DISTRIBUTION_MANIFEST_V1_PRETTYJWS => {
                let body = self.http_response.into_body();
                let bytes = hyper::body::to_bytes(body).await?;
                ResponseBody::V1PrettyJWS(serde_json::from_slice(&bytes)?)
            }
            APPLICATION_VND_DOCKER_DISTRIBUTION_MANIFEST_V2_JSON => {
                let body = self.http_response.into_body();
                let bytes = hyper::body::to_bytes(body).await?;
                ResponseBody::V2(serde_json::from_slice(&bytes)?)
            }
            APPLICATION_VND_DOCKER_DISTRIBUTION_MANIFEST_LIST_V2_JSON => {
                let body = self.http_response.into_body();
                let bytes = hyper::body::to_bytes(body).await?;
                ResponseBody::V2List(serde_json::from_slice(&bytes)?)
            }
            content_type => todo!("{content_type}"),
        })
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

///
#[derive(Debug)]
pub enum ResponseBody {
    ///
    V1(spec::v2::schema_1::ManifestResponseBody),
    ///
    V1PrettyJWS(spec::v2::schema_1::ManifestResponseBody),
    ///
    V2(spec::v2::schema_2::ManifestResponseBody),
    ///
    V2List(spec::v2::schema_2::ManifestListResponseBody),
    ///
    Error(spec::v2::ErrorResponseBody),
}
