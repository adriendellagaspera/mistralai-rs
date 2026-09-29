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
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .documents
    ///         .libraries_documents_list_v1(
    ///             &"library_id".to_string(),
    ///             &LibrariesDocumentsListV1QueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_documents_list_v1(
        &self,
        library_id: &str,
        request: &LibrariesDocumentsListV1QueryRequest,
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
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .documents
    ///         .libraries_documents_upload_v1(
    ///             &"library_id".to_string(),
    ///             &LibrariesDocumentsUploadV1Request {
    ///                 file: File("file".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_documents_upload_v1(
        &self,
        library_id: &str,
        request: &LibrariesDocumentsUploadV1Request,
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
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .documents
    ///         .libraries_documents_get_v1(&"library_id".to_string(), &"document_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_documents_get_v1(
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
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .documents
    ///         .libraries_documents_update_v1(
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
    pub async fn libraries_documents_update_v1(
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
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .documents
    ///         .libraries_documents_delete_v1(&"library_id".to_string(), &"document_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_documents_delete_v1(
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
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .documents
    ///         .libraries_documents_patch_v1(
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
    pub async fn libraries_documents_patch_v1(
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
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .documents
    ///         .libraries_documents_get_text_content_v1(
    ///             &"library_id".to_string(),
    ///             &"document_id".to_string(),
    ///             &LibrariesDocumentsGetTextContentV1QueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_documents_get_text_content_v1(
        &self,
        library_id: &str,
        document_id: &str,
        request: &LibrariesDocumentsGetTextContentV1QueryRequest,
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
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .documents
    ///         .libraries_documents_get_status_v1(
    ///             &"library_id".to_string(),
    ///             &"document_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_documents_get_status_v1(
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
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .documents
    ///         .libraries_documents_get_signed_url_v1(
    ///             &"library_id".to_string(),
    ///             &"document_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_documents_get_signed_url_v1(
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
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .documents
    ///         .libraries_documents_get_extracted_text_signed_url_v1(
    ///             &"library_id".to_string(),
    ///             &"document_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_documents_get_extracted_text_signed_url_v1(
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
    /// use adriendellagaspera_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .beta
    ///         .libraries
    ///         .documents
    ///         .libraries_documents_reprocess_v1(
    ///             &"library_id".to_string(),
    ///             &"document_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn libraries_documents_reprocess_v1(
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
}
