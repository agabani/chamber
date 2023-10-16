///
#[derive(Debug)]
pub enum ApiError {
    ///
    Http(hyper::http::Error),

    ///
    Parse(url::ParseError),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ApiError {}
