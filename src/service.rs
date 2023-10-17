use std::future::Future;

///
pub trait Service<Request> {
    ///
    type Response;

    ///
    type Error;

    ///
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    ///
    fn send(&self, request: &Request) -> Self::Future;
}