use std::{future::Future, pin::Pin};

use base64::Engine;

use crate::{distribution::error, parser::www_authenticate::Challenge};

///
#[derive(Debug, Clone)]
pub enum Authentication {
    ///
    Basic(String),
    ///
    Bearer(Bearer),
}

///
#[derive(Debug, Clone)]
pub struct Bearer {
    ///
    pub access_token: String,

    ///
    pub token: String,
}

///
#[derive(Debug, Clone)]
pub enum Credential {
    ///
    UsernamePassword(UsernamePassword),
}

///
#[derive(Debug, Clone)]
pub struct UsernamePassword {
    ///
    pub username: String,

    ///
    pub password: String,
}

///
pub trait Solver {
    ///
    fn solve(
        &self,
        challenge: &Challenge,
        credential: &Credential,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Authentication>, error::Error>>>>;
}

///
pub struct BasicSolver;

impl Solver for BasicSolver {
    fn solve(
        &self,
        challenge: &Challenge,
        credential: &Credential,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Authentication>, error::Error>>>> {
        if challenge.auth_scheme != "Basic" {
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

        Box::pin(async move { Ok(Some(authentication)) })
    }
}

impl BasicSolver {
    ///
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for BasicSolver {
    fn default() -> Self {
        Self::new()
    }
}

///
pub struct BearerSolver<S>
where
    S: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>,
{
    _client: S,
}

impl<S> BearerSolver<S>
where
    S: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>,
{
    ///
    #[must_use]
    pub fn new(client: S) -> Self {
        Self { _client: client }
    }
}

impl<S> Solver for BearerSolver<S>
where
    S: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>,
{
    fn solve(
        &self,
        _challenge: &Challenge,
        _credential: &Credential,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Authentication>, error::Error>>>> {
        let future = async move { Ok(None) };

        Box::pin(future)
    }
}
