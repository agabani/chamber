///
#[derive(serde::Deserialize, serde::Serialize)]
pub struct CatalogResponseBody {
    ///
    #[serde(rename = "repositories")]
    pub repositories: Vec<String>,
}

///
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ErrorResponseBody {
    ///
    #[serde(rename = "errors")]
    pub errors: Vec<ErrorResponseBodyError>,
}

///
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ErrorResponseBodyError {
    ///
    #[serde(rename = "code")]
    pub code: String,

    ///
    #[serde(rename = "message")]
    pub message: String,

    ///
    #[serde(rename = "detail")]
    pub detail: Vec<ErrorResponseBodyErrorDetail>,
}

///
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ErrorResponseBodyErrorDetail {
    ///
    #[serde(rename = "Type")]
    pub type_: String,

    ///
    #[serde(rename = "Class")]
    pub class: String,

    ///
    #[serde(rename = "Name")]
    pub name: String,

    ///
    #[serde(rename = "Action")]
    pub action: String,
}
