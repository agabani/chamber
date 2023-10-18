mod basic_solver;
mod bearer_solver;

use std::{future::Future, pin::Pin};

pub use basic_solver::*;
pub use bearer_solver::*;

use super::{error, www_authenticate::Challenge};

///
#[derive(Debug, Clone)]
pub enum Authentication {
    ///
    Basic(String),
    ///
    Bearer(Bearer),
}

///
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Bearer {
    ///
    #[serde(rename = "access_token")]
    pub access_token: String,

    ///
    #[serde(rename = "token")]
    pub token: String,
}

///
#[derive(Debug, Clone)]
pub enum Credential {
    ///
    UsernamePassword(UsernamePassword),
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
#[derive(Debug, Clone)]
pub struct UsernamePassword {
    ///
    pub username: String,

    ///
    pub password: String,
}
