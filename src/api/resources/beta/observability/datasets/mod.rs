use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod records;
pub use records::RecordsClient;
pub struct DatasetsClient {
    pub http_client: HttpClient,
    pub records: RecordsClient,
}

impl DatasetsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            records: RecordsClient::new(config.clone())?,
        })
    }

    /// List existing datasets
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
    ///         .observability
    ///         .datasets
    ///         .get_datasets_v1observability_datasets_get(
    ///             &GetDatasetsV1ObservabilityDatasetsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_datasets_v1observability_datasets_get(
        &self,
        request: &GetDatasetsV1ObservabilityDatasetsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDatasetsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/observability/datasets",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .serialize("q", request.q.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new empty dataset
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
    ///         .observability
    ///         .datasets
    ///         .create_dataset_v1observability_datasets_post(
    ///             &CreateDatasetRequest {
    ///                 name: "name".to_string(),
    ///                 description: "description".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_dataset_v1observability_datasets_post(
        &self,
        request: &CreateDatasetRequest,
        options: Option<RequestOptions>,
    ) -> Result<Dataset, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/datasets",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get dataset by id
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
    ///         .observability
    ///         .datasets
    ///         .get_dataset_by_id_v1observability_datasets_dataset_id_get(&"dataset_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_dataset_by_id_v1observability_datasets_dataset_id_get(
        &self,
        dataset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DatasetPreview, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/datasets/{}", dataset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a dataset
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
    ///         .observability
    ///         .datasets
    ///         .delete_dataset_v1observability_datasets_dataset_id_delete(&"dataset_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_dataset_v1observability_datasets_dataset_id_delete(
        &self,
        dataset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/observability/datasets/{}", dataset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Patch dataset
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
    ///         .observability
    ///         .datasets
    ///         .update_dataset_v1observability_datasets_dataset_id_patch(
    ///             &"dataset_id".to_string(),
    ///             &UpdateDatasetRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_dataset_v1observability_datasets_dataset_id_patch(
        &self,
        dataset_id: &str,
        request: &UpdateDatasetRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetPreview, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/observability/datasets/{}", dataset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List existing records in the dataset
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
    ///         .observability
    ///         .datasets
    ///         .get_dataset_records_v1observability_datasets_dataset_id_records_get(
    ///             &"dataset_id".to_string(),
    ///             &GetDatasetRecordsV1ObservabilityDatasetsDatasetIDRecordsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_dataset_records_v1observability_datasets_dataset_id_records_get(
        &self,
        dataset_id: &str,
        request: &GetDatasetRecordsV1ObservabilityDatasetsDatasetIdRecordsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDatasetRecordsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/datasets/{}/records", dataset_id),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a record to the dataset
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
    ///         .observability
    ///         .datasets
    ///         .create_dataset_record_v1observability_datasets_dataset_id_records_post(
    ///             &"dataset_id".to_string(),
    ///             &CreateDatasetRecordRequest {
    ///                 payload: DatasetRecordPayload(HashMap::from([(
    ///                     "key".to_string(),
    ///                     serde_json::json!("value"),
    ///                 )])),
    ///                 properties: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_dataset_record_v1observability_datasets_dataset_id_records_post(
        &self,
        dataset_id: &str,
        request: &CreateDatasetRecordRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/observability/datasets/{}/records", dataset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Populate the dataset with records from a campaign
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
    ///     client.beta.observability.datasets.post_dataset_records_from_campaign_v1observability_datasets_dataset_id_imports_from_campaign_post(&"dataset_id".to_string(), &ImportDatasetFromCampaignRequest {
    ///         campaign_id: "campaign_id".to_string()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn post_dataset_records_from_campaign_v1observability_datasets_dataset_id_imports_from_campaign_post(
        &self,
        dataset_id: &str,
        request: &ImportDatasetFromCampaignRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetImportTask, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/observability/datasets/{}/imports/from-campaign",
                    dataset_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Populate the dataset with records from the explorer
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
    ///     client.beta.observability.datasets.post_dataset_records_from_explorer_v1observability_datasets_dataset_id_imports_from_explorer_post(&"dataset_id".to_string(), &ImportDatasetFromExplorerRequest {
    ///         completion_event_ids: vec!["completion_event_ids".to_string()]
    ///     }, None).await;
    /// }
    /// ```
    pub async fn post_dataset_records_from_explorer_v1observability_datasets_dataset_id_imports_from_explorer_post(
        &self,
        dataset_id: &str,
        request: &ImportDatasetFromExplorerRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetImportTask, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/observability/datasets/{}/imports/from-explorer",
                    dataset_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Populate the dataset with records from an uploaded file
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
    ///         .observability
    ///         .datasets
    ///         .post_dataset_records_from_file_v1observability_datasets_dataset_id_imports_from_file_post(
    ///             &"dataset_id".to_string(),
    ///             &ImportDatasetFromFileRequest {
    ///                 file_id: "file_id".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn post_dataset_records_from_file_v1observability_datasets_dataset_id_imports_from_file_post(
        &self,
        dataset_id: &str,
        request: &ImportDatasetFromFileRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetImportTask, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/observability/datasets/{}/imports/from-file", dataset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Populate the dataset with records from playground conversations
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
    ///     client.beta.observability.datasets.post_dataset_records_from_playground_v1observability_datasets_dataset_id_imports_from_playground_post(&"dataset_id".to_string(), &ImportDatasetFromPlaygroundRequest {
    ///         conversation_ids: vec!["conversation_ids".to_string()]
    ///     }, None).await;
    /// }
    /// ```
    pub async fn post_dataset_records_from_playground_v1observability_datasets_dataset_id_imports_from_playground_post(
        &self,
        dataset_id: &str,
        request: &ImportDatasetFromPlaygroundRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetImportTask, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/observability/datasets/{}/imports/from-playground",
                    dataset_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Populate the dataset with records from another dataset
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
    ///     client.beta.observability.datasets.post_dataset_records_from_dataset_v1observability_datasets_dataset_id_imports_from_dataset_post(&"dataset_id".to_string(), &ImportDatasetFromDatasetRequest {
    ///         dataset_record_ids: vec!["dataset_record_ids".to_string()]
    ///     }, None).await;
    /// }
    /// ```
    pub async fn post_dataset_records_from_dataset_v1observability_datasets_dataset_id_imports_from_dataset_post(
        &self,
        dataset_id: &str,
        request: &ImportDatasetFromDatasetRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetImportTask, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/observability/datasets/{}/imports/from-dataset",
                    dataset_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Export to the Files API and retrieve presigned URL to download the resulting JSONL file
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
    ///         .observability
    ///         .datasets
    ///         .export_dataset_to_jsonl_v1observability_datasets_dataset_id_exports_to_jsonl_get(
    ///             &"dataset_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_dataset_to_jsonl_v1observability_datasets_dataset_id_exports_to_jsonl_get(
        &self,
        dataset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ExportDatasetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/datasets/{}/exports/to-jsonl", dataset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get status of a dataset import task
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
    ///         .observability
    ///         .datasets
    ///         .get_dataset_import_task_v1observability_datasets_dataset_id_tasks_task_id_get(
    ///             &"dataset_id".to_string(),
    ///             &"task_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_dataset_import_task_v1observability_datasets_dataset_id_tasks_task_id_get(
        &self,
        dataset_id: &str,
        task_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DatasetImportTask, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/datasets/{}/tasks/{}", dataset_id, task_id),
                None,
                None,
                options,
            )
            .await
    }

    /// List import tasks for the given dataset
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
    ///         .observability
    ///         .datasets
    ///         .get_dataset_import_tasks_v1observability_datasets_dataset_id_tasks_get(
    ///             &"dataset_id".to_string(),
    ///             &GetDatasetImportTasksV1ObservabilityDatasetsDatasetIDTasksGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_dataset_import_tasks_v1observability_datasets_dataset_id_tasks_get(
        &self,
        dataset_id: &str,
        request: &GetDatasetImportTasksV1ObservabilityDatasetsDatasetIdTasksGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDatasetImportTasksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/datasets/{}/tasks", dataset_id),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }
}
