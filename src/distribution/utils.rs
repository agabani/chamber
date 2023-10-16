use std::borrow::Cow;

use hyper::{body::HttpBody, Body, Response, StatusCode};

use crate::{parser::www_authenticate::WwwAuthenticate, Result};

use super::{
    api::{
        self, Catalog, CatalogRequest, CatalogResponse, Support, SupportRequest, SupportResponse,
    },
    authentication::{Authentication, Credential, Solvers},
};

///
pub async fn deserialize_response_body<'a, B>(
    response: Response<Body>,
) -> Result<B, serde_json::Error>
where
    B: serde::de::DeserializeOwned,
{
    let content_length = response
        .body()
        .size_hint()
        .upper()
        .expect("TODO: expected content length");

    if content_length > 5_000_000 {
        panic!("content length {content_length} too long")
    }

    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();

    serde_json::from_slice::<B>(&bytes)
}

///
pub async fn catalog<'a>(
    api: &Catalog,
    solvers: &Solvers,
    credential: Option<&Credential>,
    authentication: Cow<'a, Option<Authentication>>,
    request: &CatalogRequest,
) -> Result<(CatalogResponse, Cow<'a, Option<Authentication>>)> {
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

///
pub async fn tags_list<'a>(
    api: &api::TagsList,
    solvers: &Solvers,
    credential: Option<&Credential>,
    authentication: Cow<'a, Option<Authentication>>,
    request: &api::TagsListRequest,
) -> Result<(api::TagsListResponse, Cow<'a, Option<Authentication>>)> {
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
