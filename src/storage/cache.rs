use std::{io::Error, path::PathBuf};

use tokio::{
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter},
};

///
#[derive(Clone)]
pub struct Cache {
    base_directory: PathBuf,
}

impl Cache {
    ///
    pub fn new(base_directory: impl Into<PathBuf>) -> Self {
        Self {
            base_directory: base_directory.into(),
        }
    }

    ///
    pub fn blob(&self, name: &str, digest: &str) -> Blob {
        Blob {
            base_directory: self
                .base_directory
                .join(name)
                .join(digest.replace(":", "_")),
        }
    }
}

///
pub struct Blob {
    base_directory: PathBuf,
}

impl Blob {
    const FILENAME: &str = "blob";

    ///
    pub async fn reader(&self) -> Result<BufReader<File>, Error> {
        let file = File::create(self.base_directory.join(Self::FILENAME)).await?;
        Ok(BufReader::new(file))
    }

    ///
    pub async fn writer(&self) -> Result<BufWriter<File>, Error> {
        create_dir_all(&self.base_directory).await?;
        let file = File::create(self.base_directory.join(Self::FILENAME)).await?;
        Ok(BufWriter::new(file))
    }
}
