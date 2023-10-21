///
pub async fn stream<T>(mut body: hyper::body::Body, writer: &mut T) -> Result<(), StreamError>
where
    T: tokio::io::AsyncWriteExt + Unpin,
{
    use hyper::body::HttpBody;

    while let Some(data) = body.data().await {
        let mut bytes = data?;
        writer.write_all_buf(&mut bytes).await?;

        // break;
    }

    writer.flush().await?;

    Ok(())
}

///
#[derive(Debug)]
pub enum StreamError {
    ///
    Hyper(hyper::Error),
    ///
    Io(std::io::Error),
}

impl std::fmt::Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for StreamError {}

impl From<hyper::Error> for StreamError {
    fn from(value: hyper::Error) -> Self {
        StreamError::Hyper(value)
    }
}

impl From<std::io::Error> for StreamError {
    fn from(value: std::io::Error) -> Self {
        StreamError::Io(value)
    }
}
