use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct FilesClient {
    pub http_client: HttpClient,
}

impl FilesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a list of files that belong to the user's organization.
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
    ///         .files
    ///         .list(
    ///             &ListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFilesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/files",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("page_size", request.page_size.clone())
                    .bool("include_total", request.include_total.clone())
                    .serialize("sample_type", request.sample_type.clone())
                    .serialize("source", request.source.clone())
                    .serialize("search", request.search.clone())
                    .serialize("purpose", request.purpose.clone())
                    .serialize("mimetypes", request.mimetypes.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Upload a file that can be used across various endpoints.
    ///
    /// The size of individual files can be a maximum of 512 MB. The Fine-tuning API only supports .jsonl files.
    ///
    /// Please contact us if you need to increase these storage limits.
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
    ///         .files
    ///         .upload(
    ///             &UploadRequest {
    ///                 expiry: None,
    ///                 file: File("file".to_string()),
    ///                 purpose: None,
    ///                 visibility: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upload(
        &self,
        request: &UploadRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateFileResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/files",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Returns information about a specific file.
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
    ///     client.files.retrieve(&"file_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetFileResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/files/{}", file_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a file.
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
    ///     client.files.delete(&"file_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteFileResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/files/{}", file_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Download a file
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn download(
        &self,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("v1/files/{}/content", file_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get Signed Url
    ///
    /// # Arguments
    ///
    /// * `expiry` - Number of hours before the URL becomes invalid. Defaults to 24h. Must be between 1h and 168h.
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
    ///         .files
    ///         .get_signed_url(
    ///             &"file_id".to_string(),
    ///             &GetSignedURLQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_signed_url(
        &self,
        file_id: &str,
        request: &GetSignedUrlQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSignedUrlResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/files/{}/url", file_id),
                None,
                QueryBuilder::new()
                    .int("expiry", request.expiry.clone())
                    .build(),
                options,
            )
            .await
    }
}
