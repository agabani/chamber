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
    pub detail: Vec<ErrorResponseBodyErrorDetail>,

    ///
    #[serde(rename = "message")]
    pub message: String,
}

///
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ErrorResponseBodyErrorDetail {
    ///
    #[serde(rename = "Action")]
    pub action: String,

    ///
    #[serde(rename = "Class")]
    pub class: String,

    ///
    #[serde(rename = "Type")]
    pub type_: String,

    ///
    #[serde(rename = "Name")]
    pub name: String,
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
