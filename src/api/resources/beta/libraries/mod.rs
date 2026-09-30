use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod documents;
pub use documents::DocumentsClient;
pub mod accesses;
pub use accesses::AccessesClient;
pub struct LibrariesClient {
    pub http_client: HttpClient,
    pub documents: DocumentsClient,
    pub accesses: AccessesClient,
}

impl LibrariesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            documents: DocumentsClient::new(config.clone())?,
            accesses: AccessesClient::new(config.clone())?,
        })
    }

    /// List all libraries that you have created or have been shared with you.
    ///
    /// # Arguments
    ///
    /// * `page_token` - Continuation token from a previous response's next_page_token. Preferred over `page`.
    /// * `page` - Deprecated: use page_token. Offset paging re-scans earlier pages and is being phased out.
    /// * `search` - Case-insensitive search on the library name.
    /// * `filter_owned_by_me` - Deprecated: this parameter will be removed in a future version.
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
    ///         .libraries_list_v1(
    ///             &LibrariesListV1QueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_list_v1(
        &self,
        request: &LibrariesListV1QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListLibrariesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/libraries",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .serialize("page_token", request.page_token.clone())
                    .int("page", request.page.clone())
                    .serialize("search", request.search.clone())
                    .serialize("filter_owned_by_me", request.filter_owned_by_me.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new Library, you will be marked as the owner and only you will have the possibility to share it with others. When first created this will only be accessible by you.
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
    ///         .libraries_create_v1(
    ///             &CreateLibraryRequest {
    ///                 name: "name".to_string(),
    ///                 chunk_size: None,
    ///                 description: None,
    ///                 owner_type: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_create_v1(
        &self,
        request: &CreateLibraryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Library, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/libraries",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Given a library id, details information about that Library.
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
    ///         .libraries_get_v1(&"library_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_get_v1(
        &self,
        library_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Library, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/libraries/{}", library_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Given a library id, you can update the name and description.
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
    ///         .libraries_update_v1(
    ///             &"library_id".to_string(),
    ///             &UpdateLibraryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_update_v1(
        &self,
        library_id: &str,
        request: &UpdateLibraryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Library, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/libraries/{}", library_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Given a library id, deletes it together with all documents that have been uploaded to that library. Warning: the response will change from 200 (returning the deleted library) to 204 No Content in a future version.
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
    ///         .libraries_delete_v1(&"library_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_delete_v1(
        &self,
        library_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Option<Library>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/libraries/{}", library_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Given a library id, you can update the name and description.
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
    ///         .libraries_patch_v1(
    ///             &"library_id".to_string(),
    ///             &UpdateLibraryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_patch_v1(
        &self,
        library_id: &str,
        request: &UpdateLibraryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Library, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/libraries/{}", library_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
