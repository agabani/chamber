///
#[derive(Debug)]
pub enum ApiError {
    ///
    Header(hyper::header::ToStrError),

    ///
    Http(hyper::http::Error),

    ///
    Parse(url::ParseError),

    ///
    Protocol,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ApiError {}
