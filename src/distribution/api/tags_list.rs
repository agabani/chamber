use hyper::{Body, Method, Request, Response, StatusCode};
use url::Url;

use crate::{
    distribution::{authentication::Authentication, client::Client, spec, utils},
    Result,
};

///
pub struct TagsList {
    client: Client,
}

impl TagsList {
    ///
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    ///
    pub async fn send(
        &self,
        request: &TagsListRequest,
        authentication: Option<&Authentication>,
    ) -> Result<TagsListResponse> {
        let mut url = Url::parse(&request.base_url).unwrap();
        url.set_path(&format!("/v2/{}/tags/list", request.repository));

        let mut request = Request::builder().method(Method::GET).uri(url.to_string());

        if let Some(authentication) = authentication {
            let authorization = match authentication {
                Authentication::Basic(authorization) => format!("Basic {authorization}"),
                Authentication::Bearer(bearer) => format!("Bearer {}", bearer.access_token),
            };

            request = request.header("Authorization", authorization);
        }

        let request = request.body(Body::empty()).unwrap();

        let response = self.client.send(request).await.unwrap();

        Ok(TagsListResponse { raw: response })
    }
}

///
pub struct TagsListRequest {
    ///
    pub base_url: String,

    ///
    pub repository: String,
}

///
pub struct TagsListResponse {
    ///
    pub raw: Response<Body>,
}

impl TagsListResponse {
    ///
    pub async fn body(self) -> Result<TagsListResponseBody, serde_json::Error> {
        match self.raw.status() {
            StatusCode::OK => utils::deserialize_response_body(self.raw)
                .await
                .map(TagsListResponseBody::Ok),
            status_code => todo!("{status_code}"),
        }
    }
}

///
pub enum TagsListResponseBody {
    ///
    Ok(spec::TagsListResponseBody),

    ///
    Err(spec::ErrorResponseBody),
}
