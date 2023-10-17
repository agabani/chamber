use std::{future::Future, marker::PhantomData, pin::Pin};

use tower::{Service as _, ServiceExt as _};

use crate::{distribution::error, service};

///
pub type Client = tower::buffer::Buffer<
    tower::util::BoxService<
        hyper::Request<hyper::Body>,
        hyper::Response<hyper::Body>,
        tower::BoxError, // TODO: find a way to remove the boxed error...
    >,
    hyper::Request<hyper::Body>,
>;

///
pub trait Request {
    ///
    type Future: Future<Output = Result<hyper::Request<hyper::Body>, error::Error>>;

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
}

///
pub struct Service<Request, Response>
where
    Request: self::Request,
    Response: self::Response,
{
    client: Client,
    _request: PhantomData<Request>,
    _response: PhantomData<Response>,
}

impl<Request, Response> Service<Request, Response>
where
    Request: self::Request,
    Response: self::Response,
{
    ///
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _request: PhantomData,
            _response: PhantomData,
        }
    }
}

impl<Request, Response> service::Service<Request> for Service<Request, Response>
where
    Request: self::Request + 'static, // TODO: find a way to remove static...
    Response: self::Response,
{
    type Response = Response;

    type Error = error::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&self, request: Request) -> Self::Future {
        let mut client = self.client.clone();

        let future = async move {
            let http_request = request.to_http_request().await?;

            let response = client
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
