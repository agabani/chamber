use std::collections::HashMap;

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CatalogResponseBody {
    ///
    #[serde(rename = "repositories")]
    pub repositories: Vec<String>,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ErrorResponseBody {
    ///
    #[serde(rename = "errors")]
    pub errors: Vec<ErrorResponseBodyError>,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ErrorResponseBodyError {
    ///
    #[serde(rename = "code")]
    pub code: String,

    ///
    #[serde(rename = "detail")]
    pub detail: ErrorResponseBodyErrorDetail,

    ///
    #[serde(rename = "message")]
    pub message: String,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum ErrorResponseBodyErrorDetail {
    ///
    Array(Vec<HashMap<String, String>>),
    ///
    Object(HashMap<String, String>),
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SupportResponseBody {}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TagsListResponseBody {
    ///
    pub name: String,

    ///
    pub tags: Vec<String>,
}
