///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManifestListResponseBody {
    ///
    #[serde(rename = "schemaVersion")]
    pub schema_version: i32,

    /// application/vnd.docker.distribution.manifest.list.v2+json
    #[serde(rename = "mediaType")]
    pub media_type: String,

    ///
    #[serde(rename = "manifests")]
    pub manifests: Vec<ManifestListResponseBodyManifest>,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManifestListResponseBodyManifest {
    /// application/vnd.docker.distribution.manifest.v2+json
    ///
    /// application/vnd.docker.distribution.manifest.v1+json
    #[serde(rename = "mediaType")]
    pub media_type: String,

    ///
    #[serde(rename = "size")]
    pub size: u64,

    ///
    #[serde(rename = "digest")]
    pub digest: String,

    ///
    #[serde(rename = "platform")]
    pub platform: ManifestListResponseBodyManifestPlatform,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManifestListResponseBodyManifestPlatform {
    ///
    #[serde(rename = "architecture")]
    pub architecture: String,

    ///
    #[serde(rename = "os")]
    pub os: String,

    ///
    #[serde(rename = "os.version")]
    pub os_version: Option<String>,

    ///
    #[serde(rename = "os.features")]
    pub os_features: Option<Vec<String>>,

    ///
    #[serde(rename = "variant")]
    pub variant: Option<String>,

    ///
    #[serde(rename = "features")]
    pub features: Option<Vec<String>>,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManifestResponseBody {
    ///
    #[serde(rename = "schemaVersion")]
    pub schema_version: i32,

    /// application/vnd.docker.distribution.manifest.v2+json
    #[serde(rename = "mediaType")]
    pub media_type: String,

    ///
    #[serde(rename = "config")]
    pub config: ManifestResponseBodyConfig,

    ///
    #[serde(rename = "layers")]
    pub layers: Vec<ManifestResponseBodyLayer>,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManifestResponseBodyConfig {
    /// application/vnd.docker.container.image.v1+json
    #[serde(rename = "mediaType")]
    pub media_type: String,

    ///
    #[serde(rename = "size")]
    pub size: u64,

    ///
    #[serde(rename = "digest")]
    pub digest: String,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManifestResponseBodyLayer {
    /// application/vnd.docker.image.rootfs.diff.tar.gzip
    ///
    /// application/vnd.docker.image.rootfs.foreign.diff.tar.gzip
    #[serde(rename = "mediaType")]
    pub media_type: String,

    ///
    #[serde(rename = "size")]
    pub size: u64,

    ///
    #[serde(rename = "digest")]
    pub digest: String,

    ///
    #[serde(rename = "urls")]
    pub urls: Option<Vec<String>>,
}
