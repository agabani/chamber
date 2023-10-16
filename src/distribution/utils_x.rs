use std::{borrow::Cow, future::Future};

use hyper::{HeaderMap, StatusCode};

use crate::{
    distribution::{api, authentication},
    parser::www_authenticate,
};

///
pub trait ApiRequest {}

///
pub trait ApiResponse {
    ///
    fn status(&self) -> StatusCode;

    ///
    fn headers(&self) -> &HeaderMap;
}

///
pub trait Api<Request> {
    ///
    type Response;

    ///
    type Future: Future<Output = Result<Self::Response, api::ApiError>>;

    ///
    fn send(
        &self,
        request: &Request,
        authentication: Option<&authentication::Authentication>,
    ) -> Self::Future;
}

///
pub async fn send<'a, A, Request, Response>(
    api: A,
    solvers: &authentication::Solvers,
    credential: Option<&authentication::Credential>,
    authentication: Cow<'a, Option<authentication::Authentication>>,
    request: &Request,
) -> Result<(Response, Cow<'a, Option<authentication::Authentication>>), api::ApiError>
where
    A: Api<Request, Response = Response>,
    A::Response: ApiResponse,
{
    let mut response = api.send(&request, authentication.as_ref().as_ref()).await?;

    if response.status() != StatusCode::UNAUTHORIZED {
        return Ok((response, authentication));
    }

    let Some(header) = response.headers().get("Www-Authenticate") else {
        return Ok((response, authentication));
    };

    let header = header.to_str().map_err(api::ApiError::Header)?.to_string();

    let www_authenticate =
        www_authenticate::WwwAuthenticate::parse(&header).map_err(|_| api::ApiError::Protocol)?;

    let Some(credential) = credential else {
        return Ok((response, authentication));
    };

    for challenge in &www_authenticate.challenges {
        for solver in solvers.iter() {
            let authentication = solver.solve(challenge, credential).await.unwrap();

            if let Some(authentication) = authentication {
                response = api.send(request, Some(&authentication)).await?;
                if response.status() != StatusCode::UNAUTHORIZED {
                    return Ok((response, Cow::Owned(Some(authentication))));
                }
            }
        }
    }

    Ok((response, Cow::Owned(None)))
}
