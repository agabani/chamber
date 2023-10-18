use std::{future::Future, pin::Pin};

use crate::distribution::{
    error,
    service::{Request, Response},
};

///
pub struct SupportRequest;

impl Request for SupportRequest {
    type Future = Pin<Box<dyn Future<Output = Result<hyper::Request<hyper::Body>, error::Error>>>>;

    fn authentication(&self) -> Option<&crate::distribution::authentication::Authentication> {
        todo!()
    }

    fn credential(&self) -> Option<&crate::distribution::authentication::Credential> {
        todo!()
    }

    fn to_http_request(&self) -> Self::Future {
        todo!()
    }
}

///
pub struct SupportResponse;

impl Response for SupportResponse {
    type Future = Pin<Box<dyn Future<Output = Result<Self, error::Error>>>>;

    fn new(
        http_response: hyper::Response<hyper::Body>,
        authentication: Option<crate::distribution::authentication::Authentication>,
    ) -> Self::Future {
        todo!()
    }

    fn www_authenticate(
        &self,
    ) -> Result<Option<crate::parser::www_authenticate::WwwAuthenticate>, error::Error> {
        todo!()
    }
}
