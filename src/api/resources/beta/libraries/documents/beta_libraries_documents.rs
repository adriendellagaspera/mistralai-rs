use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DocumentsClient {
    pub http_client: HttpClient,
}

impl DocumentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Given a library, lists the document that have been uploaded to that library.
    ///
    /// # Arguments
    ///
    /// * `filters_attributes` - Deprecated: this parameter will be removed in a future version.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mistralai_sdk::prelude::*;
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
    ///         .documents
    ///         .list(
    ///             &"library_id".to_string(),
    ///             &BetaLibrariesDocumentsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        library_id: &str,
        request: &BetaLibrariesDocumentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDocumentsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/libraries/{}/documents", library_id),
                None,
                QueryBuilder::new()
                    .serialize("search", request.search.clone())
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .serialize("filters_attributes", request.filters_attributes.clone())
                    .string("sort_by", request.sort_by.clone())
                    .string("sort_order", request.sort_order.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Given a library, upload a new document to that library. It is queued for processing, it status will change it has been processed. The processing has to be completed in order be discoverable for the library search
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
    /// use mistralai_sdk::prelude::*;
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
    ///         .documents
    ///         .upload(
    ///             &"library_id".to_string(),
    ///             &UploadRequest {
    ///                 file: File("file".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upload(
        &self,
        library_id: &str,
        request: &UploadRequest,
        options: Option<RequestOptions>,
    ) -> Result<Document, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                &format!("v1/libraries/{}/documents", library_id),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Given a library and a document in this library, you can retrieve the metadata of that document.
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
    /// use mistralai_sdk::prelude::*;
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
    ///         .documents
    ///         .get(&"library_id".to_string(), &"document_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        library_id: &str,
        document_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Document, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/libraries/{}/documents/{}", library_id, document_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Given a library and a document in that library, update the name of that document.
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
    /// use mistralai_sdk::prelude::*;
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
    ///         .documents
    ///         .rename(
    ///             &"library_id".to_string(),
    ///             &"document_id".to_string(),
    ///             &UpdateDocumentRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn rename(
        &self,
        library_id: &str,
        document_id: &str,
        request: &UpdateDocumentRequest,
        options: Option<RequestOptions>,
    ) -> Result<Document, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/libraries/{}/documents/{}", library_id, document_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Given a library and a document in that library, delete that document. The document will be deleted from the library and the search index.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mistralai_sdk::prelude::*;
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
    ///         .documents
    ///         .delete(&"library_id".to_string(), &"document_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        library_id: &str,
        document_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/libraries/{}/documents/{}", library_id, document_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Given a library and a document in that library, update the name and/or attributes of that document.
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
    /// use mistralai_sdk::prelude::*;
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
    ///         .documents
    ///         .update(
    ///             &"library_id".to_string(),
    ///             &"document_id".to_string(),
    ///             &UpdateDocumentRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        library_id: &str,
        document_id: &str,
        request: &UpdateDocumentRequest,
        options: Option<RequestOptions>,
    ) -> Result<Document, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/libraries/{}/documents/{}", library_id, document_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Given a library and a document in that library, retrieve the signed URL of text extracted. For documents that are sent to the OCR this returns the result of the OCR queries.
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
    /// use mistralai_sdk::prelude::*;
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
    ///         .documents
    ///         .extracted_text_signed_url(&"library_id".to_string(), &"document_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn extracted_text_signed_url(
        &self,
        library_id: &str,
        document_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<String, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/libraries/{}/documents/{}/extracted-text-signed-url",
                    library_id, document_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Given a library and a document in that library, reprocess that document, it will be billed again.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mistralai_sdk::prelude::*;
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
    ///         .documents
    ///         .reprocess(&"library_id".to_string(), &"document_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn reprocess(
        &self,
        library_id: &str,
        document_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/libraries/{}/documents/{}/reprocess",
                    library_id, document_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Given a library and a document in that library, retrieve the signed URL of a specific document.The url will expire after 30 minutes and can be accessed by anyone with the link.
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
    /// use mistralai_sdk::prelude::*;
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
    ///         .documents
    ///         .get_signed_url(&"library_id".to_string(), &"document_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_signed_url(
        &self,
        library_id: &str,
        document_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<String, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/libraries/{}/documents/{}/signed-url",
                    library_id, document_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Given a library and a document in that library, retrieve the processing status of that document.
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
    /// use mistralai_sdk::prelude::*;
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
    ///         .documents
    ///         .status(&"library_id".to_string(), &"document_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn status(
        &self,
        library_id: &str,
        document_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProcessingStatus, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/libraries/{}/documents/{}/status",
                    library_id, document_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Given a library and a document in that library, you can retrieve the text content of that document if it exists. For documents like pdf, docx and pptx the text content results from our processing using Mistral OCR.
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
    /// use mistralai_sdk::prelude::*;
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
    ///         .documents
    ///         .text_content(
    ///             &"library_id".to_string(),
    ///             &"document_id".to_string(),
    ///             &TextContentQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn text_content(
        &self,
        library_id: &str,
        document_id: &str,
        request: &TextContentQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DocumentTextContent, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/libraries/{}/documents/{}/text_content",
                    library_id, document_id
                ),
                None,
                QueryBuilder::new()
                    .serialize("page_start", request.page_start.clone())
                    .serialize("page_end", request.page_end.clone())
                    .build(),
                options,
            )
            .await
    }
}
