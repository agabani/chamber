use std::{future::Future, marker::PhantomData, pin::Pin, sync::Arc};

use tower::ServiceExt as _;

use super::{authentication::Solver, error};

///
pub struct Service<Client, Request, Response>
where
    Client: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>
        + Clone
        + 'static,
    Client::Error: Into<error::Error>,
    Request: super::Request,
    Response: super::Response,
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
    Request: super::Request,
    Response: super::Response,
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

impl<Client> Service<Client, super::api::v2::catalog::Request, super::api::v2::catalog::Response>
where
    Client: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>
        + Clone
        + 'static,
    Client::Error: Into<error::Error>,
{
    ///
    pub fn v2_catalog(client: Client, solvers: Vec<Arc<dyn Solver>>) -> Self {
        Self::new(client, solvers)
    }
}

impl<Client> Service<Client, super::api::v2::support::Request, super::api::v2::support::Response>
where
    Client: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>
        + Clone
        + 'static,
    Client::Error: Into<error::Error>,
{
    ///
    pub fn v2_support(client: Client, solvers: Vec<Arc<dyn Solver>>) -> Self {
        Self::new(client, solvers)
    }
}

impl<Client>
    Service<Client, super::api::v2::tags_list::Request, super::api::v2::tags_list::Response>
where
    Client: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>
        + Clone
        + 'static,
    Client::Error: Into<error::Error>,
{
    ///
    pub fn v2_tags_list(client: Client, solvers: Vec<Arc<dyn Solver>>) -> Self {
        Self::new(client, solvers)
    }
}

impl<Client, Request, Response> crate::Service<Request> for Service<Client, Request, Response>
where
    Client: tower::Service<hyper::Request<hyper::Body>, Response = hyper::Response<hyper::Body>>
        + Clone
        + 'static,
    Client::Error: Into<error::Error>,
    Request: super::Request + 'static,
    Response: super::Response,
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
