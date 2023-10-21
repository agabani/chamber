use url::Url;

use crate::{
    distribution::{
        api::{
            self,
            v2::manifests_get::{
                APPLICATION_VND_DOCKER_DISTRIBUTION_MANIFEST_LIST_V2_JSON,
                APPLICATION_VND_DOCKER_DISTRIBUTION_MANIFEST_V2_JSON,
            },
        },
        authentication::{Authentication, Credential},
        spec,
        streaming::stream,
    },
    storage::cache::Cache,
    Service,
};

///
#[derive(Clone)]
pub struct CopyWorkflow<B, M>
where
    B: Service<api::v2::blobs_get::Request, Response = api::v2::blobs_get::Response>,
    M: Service<api::v2::manifests_get::Request, Response = api::v2::manifests_get::Response>,
{
    ///
    pub blobs: B,

    ///
    pub manifests: M,

    ///
    pub cache: Cache,
}

impl<B, M> CopyWorkflow<B, M>
where
    B: Service<api::v2::blobs_get::Request, Response = api::v2::blobs_get::Response>,
    M: Service<api::v2::manifests_get::Request, Response = api::v2::manifests_get::Response>,
{
    ///
    pub async fn execute(
        &self,
        base_url: Url,
        authentication: Option<Authentication>,
        credential: Option<Credential>,
        name: String,
        reference: String,
    ) {
        let request = api::v2::manifests_get::Request::new(
            base_url.clone(),
            authentication.clone(),
            credential.clone(),
            name.clone(),
            reference,
            vec![
                APPLICATION_VND_DOCKER_DISTRIBUTION_MANIFEST_LIST_V2_JSON.to_string(),
                APPLICATION_VND_DOCKER_DISTRIBUTION_MANIFEST_V2_JSON.to_string(),
            ],
        );

        let Ok(response) = self.manifests.call(request).await else {
            return;
        };

        let spec = response.to_spec().await.unwrap();

        match spec {
            api::v2::manifests_get::ResponseBody::V1(_) => todo!(),
            api::v2::manifests_get::ResponseBody::V1PrettyJWS(_) => todo!(),
            api::v2::manifests_get::ResponseBody::V2(spec) => {
                self.handle_manifest(base_url, authentication, credential, name, spec)
                    .await;
            }
            api::v2::manifests_get::ResponseBody::V2List(spec) => {
                self.handle_manifest_list(base_url, authentication, credential, name, spec)
                    .await;
            }
            api::v2::manifests_get::ResponseBody::Error(_) => todo!(),
        }
    }

    async fn handle_manifest_list(
        &self,
        base_url: Url,
        authentication: Option<Authentication>,
        credential: Option<Credential>,
        name: String,
        spec: spec::v2::schema_2::ManifestListResponseBody,
    ) {
        for spec in spec.manifests.into_iter() {
            self.handle_manifest_reference(
                base_url.clone(),
                authentication.clone(),
                credential.clone(),
                name.clone(),
                spec,
            )
            .await;
        }
    }

    async fn handle_manifest_reference(
        &self,
        base_url: Url,
        authentication: Option<Authentication>,
        credential: Option<Credential>,
        name: String,
        reference: spec::v2::schema_2::ManifestListResponseBodyManifest,
    ) {
        let request = api::v2::manifests_get::Request::new(
            base_url.clone(),
            authentication.clone(),
            credential.clone(),
            name.clone(),
            reference.digest,
            vec![reference.media_type],
        );

        let Ok(response) = self.manifests.call(request).await else {
            return;
        };

        let spec = response.to_spec().await.unwrap();

        let api::v2::manifests_get::ResponseBody::V2(spec) = spec else {
            return;
        };

        self.handle_manifest(base_url, authentication, credential, name, spec)
            .await;
    }

    async fn handle_manifest(
        &self,
        base_url: Url,
        authentication: Option<Authentication>,
        credential: Option<Credential>,
        name: String,
        spec: spec::v2::schema_2::ManifestResponseBody,
    ) {
        self.handle_config(
            base_url.clone(),
            authentication.clone(),
            credential.clone(),
            name.clone(),
            spec.config,
        )
        .await;

        for spec in spec.layers.into_iter() {
            self.handle_layer(
                base_url.clone(),
                authentication.clone(),
                credential.clone(),
                name.clone(),
                spec,
            )
            .await;
        }
    }

    async fn handle_config(
        &self,
        base_url: Url,
        authentication: Option<Authentication>,
        credential: Option<Credential>,
        name: String,
        spec: spec::v2::schema_2::ManifestResponseBodyConfig,
    ) {
        let request = api::v2::blobs_get::Request::new(
            base_url,
            authentication,
            credential,
            name.clone(),
            spec.digest.clone(),
        );

        let response = self.blobs.call(request).await;

        let Ok(response) = response else {
            return;
        };

        let body = response.into_body();
        let mut file = self.cache.blob(&name, &spec.digest).writer().await.unwrap();
        stream(body, &mut file).await.unwrap();
    }

    async fn handle_layer(
        &self,
        base_url: Url,
        authentication: Option<Authentication>,
        credential: Option<Credential>,
        name: String,
        spec: spec::v2::schema_2::ManifestResponseBodyLayer,
    ) {
        let request = api::v2::blobs_get::Request::new(
            base_url,
            authentication,
            credential,
            name.clone(),
            spec.digest.clone(),
        );

        let response = self.blobs.call(request).await;

        let Ok(response) = response else {
            return;
        };

        let body = response.into_body();
        let mut file = self.cache.blob(&name, &spec.digest).writer().await.unwrap();
        stream(body, &mut file).await.unwrap();
    }
}
