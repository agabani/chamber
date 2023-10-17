use std::{future::Future, marker::PhantomData, pin::Pin, sync::Arc};

use tower::ServiceExt as _;

use crate::{
    distribution::error,
    parser::www_authenticate::{self, WwwAuthenticate},
    service,
};

use super::authentication::{Authentication, Credential, Solver};

///
pub trait Request {
    ///
    type Future: Future<Output = Result<hyper::Request<hyper::Body>, error::Error>>;

    fn credential(&self) -> Option<&Credential>;

    ///
    fn to_http_request(&self) -> Self::Future;
}

///
pub trait Response
where
    Self: Sized,
{
    ///
    type Future: Future<Output = Result<Self, error::Error>>;

    ///
    fn from_http_response(response: hyper::Response<hyper::Body>) -> Self::Future; // TODO: accept successful authentication used.

    ///
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
            let http_request = request.to_http_request().await?;

            let response = client
                .ready()
                .await
                .map_err(Into::into)?
                .call(http_request)
                .await
                .map_err(Into::into)?;

            let response = Response::from_http_response(response).await?;

            let Some(www_authenticate) = response.www_authenticate()? else {
                return Ok(response);
            };

            let Some(credential) = request.credential() else {
                return Ok(response);
            };

            for challenge in www_authenticate.challenges {
                for solver in &solvers {
                    if let Some(authentication) = solver.solve(&challenge, credential).await? {
                        // ? how to add authentication ?
                        let http_request = request.to_http_request().await?;
                    }
                }
            }

            Ok(response)
        };

        Box::pin(future)
    }
}
