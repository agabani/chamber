use std::{future::Future, marker::PhantomData, pin::Pin};

use hyper::Body;
use tower::{buffer::Buffer, util::BoxService, BoxError, Service, ServiceExt};

use crate::{distribution::error, service};

///
pub type DistributionClient = Buffer<
    BoxService<hyper::Request<hyper::Body>, hyper::Response<hyper::Body>, BoxError>, // TODO: find a way to remove the boxed error...
    hyper::Request<hyper::Body>,
>;

///
pub struct DistributionService<Request, Response>
where
    Request: DistributionRequest,
    Response: DistributionResponse,
{
    inner: DistributionClient,
    _request: PhantomData<Request>,
    _response: PhantomData<Response>,
}

impl<Request, Response> service::Service<Request> for DistributionService<Request, Response>
where
    Request: DistributionRequest + 'static, // TODO: find a way to remove static...
    Response: DistributionResponse,
{
    type Response = Response;

    type Error = error::DistributionError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&self, request: Request) -> Self::Future {
        let mut inner = self.inner.clone();

        let future = async move {
            let http_request = request.to_http_request().await?;

            let response = inner
                .ready()
                .await
                .expect("TODO: Self::Error")
                .call(http_request)
                .await
                .expect("TODO: Self::Error");

            let response = Response::from_http_response(response).await?;

            // TODO: repeat request with new authentication if required

            Ok(response)
        };

        Box::pin(future)
    }
}

///
pub trait DistributionRequest {
    ///
    type Future: Future<Output = Result<hyper::Request<hyper::Body>, error::DistributionError>>;

    ///
    fn to_http_request(&self) -> Self::Future;
}

///
pub trait DistributionResponse
where
    Self: Sized,
{
    ///
    type Future: Future<Output = Result<Self, error::DistributionError>>;

    ///
    fn from_http_response(response: hyper::Response<Body>) -> Self::Future; // TODO: accept successful authentication used.
}
