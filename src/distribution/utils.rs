use std::borrow::Cow;

use hyper::StatusCode;

use crate::{parser::www_authenticate::WwwAuthenticate, Result};

use super::{
    api::{Support, SupportRequest, SupportResponse},
    authentication::{Authentication, Credential, Solvers},
};

///
pub async fn support<'a>(
    api: &Support,
    solvers: &Solvers,
    credential: Option<&Credential>,
    authentication: Cow<'a, Option<Authentication>>,
    request: &SupportRequest,
) -> Result<(SupportResponse, Cow<'a, Option<Authentication>>)> {
    let mut response = api.send(request, authentication.as_ref().as_ref()).await?;

    if response.raw.status() != StatusCode::UNAUTHORIZED {
        return Ok((response, authentication));
    }

    let header = response
        .raw
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
                    response = api.send(request, Some(&authentication)).await?;
                    if response.raw.status() != StatusCode::UNAUTHORIZED {
                        return Ok((response, Cow::Owned(Some(authentication))));
                    }
                }
            }
        }
    }

    Ok((response, Cow::Owned(None)))
}
