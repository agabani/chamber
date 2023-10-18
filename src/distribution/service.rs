use std::{future::Future, marker::PhantomData, pin::Pin, sync::Arc};

use tower::ServiceExt as _;

use crate::{
    distribution::{error, www_authenticate::WwwAuthenticate},
    service,
};

use super::authentication::{Authentication, Credential, Solver};

///
pub trait Request {
    ///
    type Future: Future<Output = Result<hyper::Request<hyper::Body>, error::Error>>;

    ///
    fn authentication(&self) -> Option<&Authentication>;

    ///
    fn credential(&self) -> Option<&Credential>;

    ///
    fn to_http_request(&self, authentication: Option<&Authentication>) -> Self::Future;
}

///
pub trait Response
where
    Self: Sized,
{
    ///
    type Future: Future<Output = Result<Self, error::Error>>;

    ///
    fn new(
        http_response: hyper::Response<hyper::Body>,
        authentication: Option<Authentication>,
    ) -> Self::Future;

    /// # Errors
    ///
    /// Will return `Err` if Www-Authenticate header is unparsable.
    fn www_authenticate(&self) -> Result<Option<WwwAuthenticate>, error::Error>;
}

///
pub struct Service<Client, Request, Response>
where
    Client: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>
        + Clone
        + 'static,
    Client::Error: Into<error::Error>,
    Request: self::Request,
    Response: self::Response,
{
    client: Client,
    solvers: Vec<Arc<dyn Solver>>,
    _request: PhantomData<Request>,
    _response: PhantomData<Response>,
}

impl<Client, Request, Response> Service<Client, Request, Response>
where
    Client: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>
        + Clone
        + 'static,
    Client::Error: Into<error::Error>,
    Request: self::Request,
    Response: self::Response,
{
    ///
    pub fn new(client: Client, solvers: Vec<Arc<dyn Solver>>) -> Self {
        Self {
            client,
            solvers,
            _request: PhantomData,
            _response: PhantomData,
        }
    }
}

impl<Client, Request, Response> service::Service<Request> for Service<Client, Request, Response>
where
    Client: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>
        + Clone
        + 'static,
    Client::Error: Into<error::Error>,
    Request: self::Request + 'static, // TODO: find a way to remove static...
    Response: self::Response,
{
    type Response = Response;

    type Error = error::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&self, request: Request) -> Self::Future {
        let mut client = self.client.clone();
        let solvers = self.solvers.clone();

        let future = async move {
            let http_request = request.to_http_request(None).await?;

            let http_response = client
                .ready()
                .await
                .map_err(Into::into)?
                .call(http_request)
                .await
                .map_err(Into::into)?;

            let response = Response::new(http_response, request.authentication().cloned()).await?;

            let Some(www_authenticate) = response.www_authenticate()? else {
                return Ok(response);
            };

            let Some(credential) = request.credential() else {
                return Ok(response);
            };

            for challenge in www_authenticate.challenges {
                for solver in &solvers {
                    if let Some(authentication) = solver.solve(&challenge, credential).await? {
                        let http_request = request.to_http_request(Some(&authentication)).await?;

                        let http_response = client
                            .ready()
                            .await
                            .map_err(Into::into)?
                            .call(http_request)
                            .await
                            .map_err(Into::into)?;

                        let response = Response::new(http_response, Some(authentication)).await?;

                        if response.www_authenticate()?.is_none() {
                            return Ok(response);
                        }
                    }
                }
            }

            Ok(response)
        };

        Box::pin(future)
    }
}
