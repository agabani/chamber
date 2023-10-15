use base64::{engine::general_purpose, Engine as _};

use crate::parser::www_authenticate::{AuthParam, WwwAuthenticate};

use super::client::Client;

///
pub struct AuthenticationChallengeSolver {
    client: Client,
}

impl AuthenticationChallengeSolver {
    ///
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    ///
    pub async fn solve(
        &self,
        www_authenticate: &WwwAuthenticate<'_>,
        credential: &Credential,
    ) -> Result<Authentication, ()> {
        for challenge in &www_authenticate.challenges {
            match challenge.auth_scheme {
                "Basic" => match self.solve_basic(credential, &challenge.auth_params).await {
                    Ok(authentication) => return Ok(authentication),
                    Err(_) => todo!(),
                },
                "Bearer" => {
                    todo!()
                }
                _ => todo!(),
            }
        }

        return Err(());
    }

    async fn solve_basic(
        &self,
        credential: &Credential,
        _auth_params: &[AuthParam<'_>],
    ) -> Result<Authentication, ()> {
        let engine = general_purpose::STANDARD;

        match credential {
            Credential::UsernamePassword(username, password) => {
                let encoded = engine.encode(format!("{username}:{password}"));
                Ok(Authentication::Basic(encoded))
            }
        }
    }

    async fn solve_bearer(
        &self,
        credential: &Credential,
        auth_params: &[AuthParam<'_>],
    ) -> Result<Authentication, ()> {
        todo!()
    }
}

///
pub enum Credential {
    ///
    UsernamePassword(String, String),
}

///
pub enum Authentication {
    ///
    Basic(String),
    ///
    Bearer,
}
