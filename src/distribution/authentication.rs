use std::{future::Future, pin::Pin};

use base64::{engine::general_purpose, Engine as _};
use hyper::{body::to_bytes, Body, Request, StatusCode};
use url::Url;

use crate::parser::www_authenticate::Challenge;

use super::client::Client;

/// Credential.
#[derive(Clone)]
pub enum Credential {
    /// Username and password.
    UsernamePassword(String, String),
}

/// Authentication.
pub enum Authentication {
    /// Basic.
    Basic(String),
    /// Bearer.
    Bearer(Bearer),
}

/// Solves authentication challenges
pub trait Solver {
    /// Solves a [`Challenge`].
    fn solve(
        &self,
        challenge: &Challenge,
        credential: &Credential,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Authentication>, ()>>>>;
}

/// Basic solver.
pub struct BasicSolver;

impl Solver for BasicSolver {
    fn solve(
        &self,
        challenge: &Challenge,
        credential: &Credential,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Authentication>, ()>>>> {
        let result = if challenge.auth_scheme.as_ref() == "Basic" {
            match credential {
                Credential::UsernamePassword(username, password) => {
                    let engine = general_purpose::STANDARD;
                    let encoded = engine.encode(format!("{username}:{password}"));
                    Ok(Some(Authentication::Basic(encoded)))
                }
            }
        } else {
            Ok(None)
        };

        Box::pin(async move { result })
    }
}

/// Bearer Solver.
pub struct BearerSolver {
    client: Client,
}

impl BearerSolver {
    /// Create a [`BearerSolver`].
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl Solver for BearerSolver {
    fn solve(
        &self,
        challenge: &Challenge,
        credential: &Credential,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Authentication>, ()>>>> {
        if challenge.auth_scheme.as_ref() != "Bearer" {
            return Box::pin(async move { Ok(None) });
        }

        let header = match credential {
            Credential::UsernamePassword(username, password) => {
                let engine = general_purpose::STANDARD;
                let encoded = engine.encode(format!("{username}:{password}"));
                format!("Basic {encoded}")
            }
        };

        let mut url = challenge
            .auth_params
            .iter()
            .find_map(|auth_param| {
                if auth_param.key == "realm" {
                    Some(Url::parse(auth_param.value.as_ref()).unwrap())
                } else {
                    None
                }
            })
            .unwrap();

        for auth_param in &challenge.auth_params {
            if auth_param.key != "realm" {
                url.query_pairs_mut()
                    .append_pair(auth_param.key.as_ref(), auth_param.value.as_ref());
            }
        }

        let request = Request::builder()
            .uri(url.to_string())
            .header("Authorization", header)
            .body(Body::empty())
            .unwrap();

        let client = self.client.clone();

        Box::pin(async move {
            let response = client.send(request).await.unwrap();

            if response.status() != StatusCode::OK {
                return Ok(None);
            }

            let body = response.into_body();

            let x = to_bytes(body).await.unwrap();

            let y: Bearer = serde_json::from_slice::<Bearer>(x.as_ref()).unwrap();

            Ok(Some(Authentication::Bearer(y)))
        })
    }
}

///
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Bearer {
    ///
    #[serde(rename = "access_token")]
    pub access_token: String,

    ///
    #[serde(rename = "token")]
    pub token: String,
}
