use std::{future::Future, pin::Pin};

use tower::{Layer, Service};

#[allow(clippy::module_name_repetitions)]
/// Layer to authenticate requests.
pub struct AuthenticationLayer;

impl<S> Layer<S> for AuthenticationLayer {
    type Service = AuthenticationService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthenticationService { inner }
    }
}

/// Service to authenticate requests.
#[allow(clippy::module_name_repetitions)]
pub struct AuthenticationService<S> {
    inner: S,
}

impl<S, Request> Service<Request> for AuthenticationService<S>
where
    S: Service<Request>,
    S::Future: 'static,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let fut = self.inner.call(req);

        let f = async move { fut.await };

        Box::pin(f)
    }
}
