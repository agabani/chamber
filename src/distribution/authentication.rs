use std::{future::Future, pin::Pin};

use crate::{distribution::error, parser::www_authenticate::Challenge};

///
pub enum Authentication {
    ///
    Basic(String),
    ///
    Bearer(Bearer),
}

///
pub struct Bearer {
    ///
    pub access_token: String,

    ///
    pub token: String,
}

///
pub enum Credential {
    ///
    UsernamePassword(UsernamePassword),
}

///
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
        let future = async move { Ok(None) };

        Box::pin(future)
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
    client: S,
}

impl<S> BearerSolver<S>
where
    S: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>,
{
    ///
    #[must_use]
    pub fn new(client: S) -> Self {
        Self { client }
    }
}

impl<S> Solver for BearerSolver<S>
where
    S: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>,
{
    fn solve(
        &self,
        challenge: &Challenge,
        credential: &Credential,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Authentication>, error::Error>>>> {
        let future = async move { Ok(None) };

        Box::pin(future)
    }
}
