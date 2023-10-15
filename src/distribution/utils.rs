use std::borrow::Cow;

use hyper::{Body, Response, StatusCode};

use crate::{parser::www_authenticate::WwwAuthenticate, Result};

use super::{
    api::Support,
    authentication::{Authentication, Credential, Solvers},
};

///
pub async fn support<'a>(
    api: Support,
    solvers: &Solvers,
    credential: &Option<Credential>,
    authentication: Cow<'a, Option<Authentication>>,
    base_url: &str,
) -> Result<(Response<Body>, Cow<'a, Option<Authentication>>)> {
    let mut response = api.send(base_url, authentication.as_ref()).await?;

    if response.status() != StatusCode::UNAUTHORIZED {
        return Ok((response, authentication));
    }

    let header = response
        .headers()
        .get("Www-Authenticate")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let www_authenticate = WwwAuthenticate::parse(&header).unwrap();

    if let Some(credential) = credential {
        for challenge in &www_authenticate.challenges {
            for solver in solvers.iter() {
                let authentication = solver.solve(challenge, credential).await.unwrap();

                if let Some(authentication) = authentication {
                    response = api.send(base_url, &Some(authentication.clone())).await?;
                    if response.status() != StatusCode::UNAUTHORIZED {
                        return Ok((response, Cow::Owned(Some(authentication))));
                    }
                }
            }
        }
    }

    Ok((response, Cow::Owned(None)))
}
