use std::{future::Future, pin::Pin};

use hyper::{Body, Request, Response};
use tower::Service;

///
pub struct SupportService<T> {
    inner: T,
}

impl<S> Service<SupportRequest> for SupportService<S>
where
    S: Service<Request<Body>, Response = Response<Body>>,
    S::Error: Into<crate::Error>,
    S::Future: 'static,
{
    type Response = SupportResponse;

    type Error = crate::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: SupportRequest) -> Self::Future {
        let request = req.request();

        let request = match request {
            Ok(request) => request,
            Err(err) => {
                todo!("{err}");
            }
        };

        let fut = self.inner.call(request);

        let f = async move {
            let result = fut.await;

            result
                .map(|response| {
                    let (parts, _) = response.into_parts();
                    SupportResponse { parts }
                })
                .map_err(Into::into)
        };

        Box::pin(f)
    }
}

///
pub struct SupportRequest {
    inner: hyper::http::request::Builder,
}

impl SupportRequest {
    /// Create a [`SupportRequest`].
    pub fn new() -> Self {
        Self {
            inner: Request::builder(),
        }
    }

    ///
    pub fn base_url(self, value: &str) -> Self {
        Self {
            inner: self.inner.uri(format!("{value}/v2/")),
        }
    }

    fn request(self) -> Result<Request<Body>, hyper::http::Error> {
        self.inner.body(hyper::Body::empty())
    }
}

///
pub struct SupportResponse {
    parts: hyper::http::response::Parts,
}

impl SupportResponse {
    ///
    pub fn status(&self) -> hyper::http::StatusCode {
        self.parts.status
    }
}

///
pub struct SupportLayer;

impl<S> tower::Layer<S> for SupportLayer {
    type Service = SupportService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SupportService { inner }
    }
}
