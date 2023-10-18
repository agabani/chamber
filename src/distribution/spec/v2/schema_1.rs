///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManifestResponseBody {
    ///
    #[serde(rename = "name")]
    pub name: String,

    ///
    #[serde(rename = "tag")]
    pub tag: String,

    ///
    #[serde(rename = "architecture")]
    pub architecture: String,

    ///
    #[serde(rename = "fsLayers")]
    pub fs_layers: Vec<ManifestResponseBodyFsLayer>,

    ///
    #[serde(rename = "history")]
    pub history: Vec<ManifestResponseBodyHistory>,

    ///
    #[serde(rename = "schemaVersion")]
    pub schema_version: i32,

    ///
    #[serde(rename = "signatures")]
    pub signatures: Option<Vec<ManifestResponseBodySignature>>,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManifestResponseBodyFsLayer {
    ///
    #[serde(rename = "blobSum")]
    pub blob_sum: String,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManifestResponseBodyHistory {
    ///
    #[serde(rename = "v1Compatibility")]
    pub v1_compatibility: String,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManifestResponseBodySignature {
    ///
    #[serde(rename = "header")]
    pub header: ManifestResponseBodySignatureHeader,

    ///
    #[serde(rename = "signature")]
    pub signature: String,

    ///
    #[serde(rename = "protected")]
    pub protected: String,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManifestResponseBodySignatureHeader {
    ///
    #[serde(rename = "jwk")]
    pub jwk: ManifestResponseBodySignatureHeaderJwk,

    ///
    #[serde(rename = "alg")]
    pub alg: String,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ManifestResponseBodySignatureHeaderJwk {
    ///
    #[serde(rename = "crv")]
    pub crv: String,

    ///
    #[serde(rename = "kid")]
    pub kid: String,

    ///
    #[serde(rename = "kty")]
    pub kty: String,

    ///
    #[serde(rename = "x")]
    pub x: String,

    ///
    #[serde(rename = "y")]
    pub y: String,
}
