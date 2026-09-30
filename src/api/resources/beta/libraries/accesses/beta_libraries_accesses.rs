use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AccessesClient {
    pub http_client: HttpClient,
}

impl AccessesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Given a library, list all of the Entity that have access and to what level.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .accesses
    ///         .libraries_share_list_v1(&"library_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_share_list_v1(
        &self,
        library_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListSharingResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/libraries/{}/share", library_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Given a library id, you can create or update the access level of an entity. You have to be owner of the library to share a library. An owner cannot change their own role. A library cannot be shared outside of the organization.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .accesses
    ///         .libraries_share_create_v1(
    ///             &"library_id".to_string(),
    ///             &SharingRequest {
    ///                 level: ShareEnum::Viewer,
    ///                 share_with_type: EntityType::User,
    ///                 share_with_uuid: "share_with_uuid".to_string(),
    ///                 org_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_share_create_v1(
        &self,
        library_id: &str,
        request: &SharingRequest,
        options: Option<RequestOptions>,
    ) -> Result<Sharing, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/libraries/{}/share", library_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Given a library id, you can delete the access level of an entity. An owner cannot delete their own access. You have to be the owner of the library to delete an access other than yours. Warning: the response will change from 200 (returning the deleted sharing) to 204 No Content in a future version.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Mistral::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .accesses
    ///         .libraries_share_delete_v1(
    ///             &"library_id".to_string(),
    ///             &SharingDelete {
    ///                 share_with_type: EntityType::User,
    ///                 share_with_uuid: "share_with_uuid".to_string(),
    ///                 org_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_share_delete_v1(
        &self,
        library_id: &str,
        request: &SharingDelete,
        options: Option<RequestOptions>,
    ) -> Result<Option<Sharing>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/libraries/{}/share", library_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
