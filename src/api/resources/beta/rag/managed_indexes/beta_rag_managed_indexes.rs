use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ManagedIndexesClient {
    pub http_client: HttpClient,
}

impl ManagedIndexesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists all managed search indexes for the current workspace.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Maximum number of indexes to return
    /// * `page_token` - Cursor returned as next_page_token by the previous page
    /// * `name` - Case-insensitive substring to match against index names
    /// * `status` - Status to match
    /// * `creator_id` - Creator ID to match
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
    ///         .rag
    ///         .managed_indexes
    ///         .list_indexes_v1rag_managed_indexes_get(
    ///             &ListIndexesV1RagManagedIndexesGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_indexes_v1rag_managed_indexes_get(
        &self,
        request: &ListIndexesV1RagManagedIndexesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListManagedIndexesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/rag/managed_indexes",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .serialize("page_token", request.page_token.clone())
                    .serialize("name", request.name.clone())
                    .serialize("status", request.status.clone())
                    .serialize("creator_id", request.creator_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Reserves a managed search index and starts asynchronous backing-table provisioning. Poll the returned Location until status is ready or failed.
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
    ///         .rag
    ///         .managed_indexes
    ///         .create_index_v1rag_managed_indexes_post(
    ///             &CreateManagedIndexRequest {
    ///                 config: ManagedIndexConfig {
    ///                     embedding: ManagedIndexConfigEmbedding::Custom {
    ///                         data: CustomEmbeddingModel {
    ///                             dimensions: 1,
    ///                             name: "name".to_string(),
    ///                             ..Default::default()
    ///                         },
    ///                     },
    ///                 },
    ///                 name: "name".to_string(),
    ///                 schema: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_index_v1rag_managed_indexes_post(
        &self,
        request: &CreateManagedIndexRequest,
        options: Option<RequestOptions>,
    ) -> Result<ManagedIndexResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/rag/managed_indexes",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the managed search index with the given name.
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
    ///         .rag
    ///         .managed_indexes
    ///         .get_index_v1rag_managed_indexes_index_name_get(&"index_name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_index_v1rag_managed_indexes_index_name_get(
        &self,
        index_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<ManagedIndexResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/rag/managed_indexes/{}", index_name),
                None,
                None,
                options,
            )
            .await
    }

    /// Records a new schema for the managed search index and applies it asynchronously. The index keeps serving its current schema until the new one is on disk; poll it until status is ready. Additive only: the new schema must keep every field the current one has.
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
    ///         .rag
    ///         .managed_indexes
    ///         .update_index_v1rag_managed_indexes_index_name_put(
    ///             &"index_name".to_string(),
    ///             &UpdateManagedIndexRequest {
    ///                 schema: ManagedIndexFields {
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_index_v1rag_managed_indexes_index_name_put(
        &self,
        index_name: &str,
        request: &UpdateManagedIndexRequest,
        options: Option<RequestOptions>,
    ) -> Result<ManagedIndexResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/rag/managed_indexes/{}", index_name),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Marks the managed search index for deletion and returns it with status deleting. The backing table is dropped asynchronously; poll the index until it returns 404.
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
    ///         .rag
    ///         .managed_indexes
    ///         .delete_index_v1rag_managed_indexes_index_name_delete(&"index_name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_index_v1rag_managed_indexes_index_name_delete(
        &self,
        index_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteManagedIndexResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/rag/managed_indexes/{}", index_name),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns a single chunk by its canonical id. A 404 means no chunk with that id exists in the index.
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
    ///         .rag
    ///         .managed_indexes
    ///         .get_chunk_index_v1rag_managed_indexes_index_name_chunks_chunk_id_get(
    ///             &"index_name".to_string(),
    ///             &"chunk_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_chunk_index_v1rag_managed_indexes_index_name_chunks_chunk_id_get(
        &self,
        index_name: &str,
        chunk_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<NavigationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/rag/managed_indexes/{}/chunks/{}", index_name, chunk_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Validates and persists documents. Always returns 200; each document is reported as accepted or rejected in the response body.
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
    ///         .rag
    ///         .managed_indexes
    ///         .ingest_documents_v1rag_managed_indexes_index_name_documents_post(
    ///             &"index_name".to_string(),
    ///             &IngestDocumentsRequest {
    ///                 documents: vec![HashMap::from([(
    ///                     "key".to_string(),
    ///                     serde_json::json!("value"),
    ///                 )])],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn ingest_documents_v1rag_managed_indexes_index_name_documents_post(
        &self,
        index_name: &str,
        request: &IngestDocumentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<IngestDocumentsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/rag/managed_indexes/{}/documents", index_name),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes the given documents by id. Returns the ids actually removed and, in `missing`, any requested ids that were not present.
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
    ///         .rag
    ///         .managed_indexes
    ///         .delete_documents_v1rag_managed_indexes_index_name_documents_delete(
    ///             &"index_name".to_string(),
    ///             &DeleteDocumentsRequest {
    ///                 document_ids: vec!["document_ids".to_string()],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_documents_v1rag_managed_indexes_index_name_documents_delete(
        &self,
        index_name: &str,
        request: &DeleteDocumentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteDocumentsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/rag/managed_indexes/{}/documents", index_name),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lexical substring match within a single source, in reading order. PHRASE mode matches the pattern literally (whitespace-sensitive, case-insensitive); TERM mode requires every whitespace-split token present in any order. A 200 with an empty list means the source exists but no chunk matched; a 404 means the source has no chunks at all.
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
    ///         .rag
    ///         .managed_indexes
    ///         .grep_index_v1rag_managed_indexes_index_name_grep_post(
    ///             &"index_name".to_string(),
    ///             &GrepRequest {
    ///                 pattern: "pattern".to_string(),
    ///                 source_id: "source_id".to_string(),
    ///                 content_type: None,
    ///                 mode: None,
    ///                 top_k: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn grep_index_v1rag_managed_indexes_index_name_grep_post(
        &self,
        index_name: &str,
        request: &GrepRequest,
        options: Option<RequestOptions>,
    ) -> Result<NavigationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/rag/managed_indexes/{}/grep", index_name),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the chunks adjacent to a position within a source, in reading order. NEXT fetches chunks at or after the end offset; PREVIOUS fetches chunks before the start offset. A 200 with an empty list means the source exists but no chunk falls in the requested direction; a 404 means the source has no chunks at all.
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
    ///         .rag
    ///         .managed_indexes
    ///         .navigate_index_v1rag_managed_indexes_index_name_navigate_post(
    ///             &"index_name".to_string(),
    ///             &NavigateRequest {
    ///                 direction: NavigationDirection::Next,
    ///                 end_offset: 1,
    ///                 source_id: "source_id".to_string(),
    ///                 start_offset: 1,
    ///                 content_type: None,
    ///                 top_k: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn navigate_index_v1rag_managed_indexes_index_name_navigate_post(
        &self,
        index_name: &str,
        request: &NavigateRequest,
        options: Option<RequestOptions>,
    ) -> Result<NavigationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/rag/managed_indexes/{}/navigate", index_name),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the chunks of a source whose span falls within [start_offset, end_offset). Either bound may be omitted to leave that side open. A 200 with an empty list means the source exists but no chunk falls in the range; a 404 means the source has no chunks at all.
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
    ///         .rag
    ///         .managed_indexes
    ///         .read_index_v1rag_managed_indexes_index_name_read_post(
    ///             &"index_name".to_string(),
    ///             &ReadRequest {
    ///                 source_id: "source_id".to_string(),
    ///                 content_type: None,
    ///                 end_offset: None,
    ///                 start_offset: None,
    ///                 top_k: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn read_index_v1rag_managed_indexes_index_name_read_post(
        &self,
        index_name: &str,
        request: &ReadRequest,
        options: Option<RequestOptions>,
    ) -> Result<NavigationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/rag/managed_indexes/{}/read", index_name),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Runs an approximate-nearest-neighbor vector search over the index and returns matching chunks, closest-first.
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
    ///         .rag
    ///         .managed_indexes
    ///         .search_index_v1rag_managed_indexes_index_name_search_post(
    ///             &"index_name".to_string(),
    ///             &SearchRequest {
    ///                 retriever: SearchRequestRetriever::Keyword {
    ///                     data: KeywordRetriever {
    ///                         query: "query".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_index_v1rag_managed_indexes_index_name_search_post(
        &self,
        index_name: &str,
        request: &SearchRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/rag/managed_indexes/{}/search", index_name),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
