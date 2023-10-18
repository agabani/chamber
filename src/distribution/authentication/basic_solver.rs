use std::{future::Future, pin::Pin};

use base64::Engine;

use crate::distribution::{error, www_authenticate::Challenge};

use super::{Authentication, Credential, Solver};

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
