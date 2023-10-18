use std::{future::Future, pin::Pin};

use base64::Engine;
use hyper::StatusCode;
use tower::ServiceExt;

use crate::distribution::{error, www_authenticate::Challenge};

use super::{Authentication, Bearer, Credential, Solver};

///
pub struct BearerSolver<S>
where
    S: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>
        + Clone
        + 'static,
    S::Error: Into<error::Error>,
{
    client: S,
}

impl<S> BearerSolver<S>
where
    S: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>
        + Clone
        + 'static,
    S::Error: Into<error::Error>,
{
    ///
    #[must_use]
    pub fn new(client: S) -> Self {
        Self { client }
    }
}

impl<S> Solver for BearerSolver<S>
where
    S: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>
        + Clone
        + 'static,
    S::Error: Into<error::Error>,
{
    fn solve(
        &self,
        challenge: &Challenge,
        credential: &Credential,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Authentication>, error::Error>>>> {
        if challenge.auth_scheme != "Bearer" {
            return Box::pin(async move { Ok(None) });
        }

        let authentication = match credential {
            Credential::UsernamePassword(credential) => {
                let engine = base64::engine::general_purpose::STANDARD;
                let encoded =
                    engine.encode(format!("{}:{}", credential.username, credential.password));
                Authentication::Basic(encoded)
            }
        };

        let Some(realm_param) = challenge
            .auth_params
            .iter()
            .find(|auth_param| auth_param.key == "realm")
        else {
            return Box::pin(async move {
                Err(error::Error::Protocol(
                    "Www-Authenticate challenge did not contain realm.".to_string(),
                ))
            });
        };

        let mut url = match url::Url::parse(&realm_param.value) {
            Ok(url) => url,
            Err(error) => return Box::pin(async move { Err(error.into()) }),
        };

        for auth_param in challenge
            .auth_params
            .iter()
            .filter(|auth_param| auth_param.key != "realm")
        {
            url.query_pairs_mut()
                .append_pair(auth_param.key.as_ref(), auth_param.value.as_ref());
        }

        let request = hyper::Request::builder()
            .uri(url.to_string())
            .header(
                "Authorization",
                match &authentication {
                    Authentication::Basic(token) => format!("Basic {token}"),
                    Authentication::Bearer(_bearer) => todo!("TODO: Bearer refresh token."),
                },
            )
            .body(hyper::Body::empty())
            .unwrap();

        let mut client = self.client.clone();

        let future = async move {
            let http_response = client
                .ready()
                .await
                .map_err(Into::into)?
                .call(request)
                .await
                .map_err(Into::into)?;

            if http_response.status() != StatusCode::OK {
                return Ok(None);
            }

            let bytes = hyper::body::to_bytes(http_response.into_body()).await?;
            let bearer = serde_json::from_slice::<Bearer>(&bytes)?;

            Ok(Some(Authentication::Bearer(bearer)))
        };

        Box::pin(future)
    }
}
