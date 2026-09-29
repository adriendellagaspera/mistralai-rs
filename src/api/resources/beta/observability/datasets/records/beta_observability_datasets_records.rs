use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct RecordsClient {
    pub http_client: HttpClient,
}

impl RecordsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get the content of a given dataset record
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
    ///         .records
    ///         .get_dataset_record_v1observability_dataset_records_dataset_record_id_get(
    ///             &"dataset_record_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_dataset_record_v1observability_dataset_records_dataset_record_id_get(
        &self,
        dataset_record_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DatasetRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/dataset-records/{}", dataset_record_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a record from a dataset
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
    ///         .records
    ///         .delete_dataset_record_v1observability_dataset_records_dataset_record_id_delete(
    ///             &"dataset_record_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_dataset_record_v1observability_dataset_records_dataset_record_id_delete(
        &self,
        dataset_record_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/observability/dataset-records/{}", dataset_record_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete multiple records from datasets
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
    ///         .records
    ///         .delete_dataset_records_v1observability_dataset_records_bulk_delete_post(
    ///             &DeleteDatasetRecordsRequest {
    ///                 dataset_record_ids: vec!["dataset_record_ids".to_string()],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_dataset_records_v1observability_dataset_records_bulk_delete_post(
        &self,
        request: &DeleteDatasetRecordsRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/observability/dataset-records/bulk-delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Run Judge on a dataset record based on the given options
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
    ///         .records
    ///         .judge_dataset_record_v1observability_dataset_records_dataset_record_id_live_judging_post(
    ///             &"dataset_record_id".to_string(),
    ///             &JudgeDatasetRecordRequest {
    ///                 judge_definition: CreateJudgeRequest {
    ///                     name: "name".to_string(),
    ///                     description: "description".to_string(),
    ///                     model_name: "model_name".to_string(),
    ///                     output: CreateJudgeRequestOutput::Classification {
    ///                         data: JudgeClassificationOutput {
    ///                             options: vec![JudgeClassificationOutputOption {
    ///                                 value: "value".to_string(),
    ///                                 description: "description".to_string(),
    ///                                 ..Default::default()
    ///                             }],
    ///                             ..Default::default()
    ///                         },
    ///                     },
    ///                     instructions: "instructions".to_string(),
    ///                     tools: vec!["tools".to_string()],
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn judge_dataset_record_v1observability_dataset_records_dataset_record_id_live_judging_post(
        &self,
        dataset_record_id: &str,
        request: &JudgeDatasetRecordRequest,
        options: Option<RequestOptions>,
    ) -> Result<JudgeOutput, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/observability/dataset-records/{}/live-judging",
                    dataset_record_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Update a dataset record payload
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
    ///     client.beta.observability.datasets.records.update_dataset_record_payload_v1observability_dataset_records_dataset_record_id_payload_put(&"dataset_record_id".to_string(), &UpdateDatasetRecordPayloadRequest {
    ///         payload: DatasetRecordPayload(HashMap::from([("key".to_string(), serde_json::json!("value"))]))
    ///     }, None).await;
    /// }
    /// ```
    pub async fn update_dataset_record_payload_v1observability_dataset_records_dataset_record_id_payload_put(
        &self,
        dataset_record_id: &str,
        request: &UpdateDatasetRecordPayloadRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "v1/observability/dataset-records/{}/payload",
                    dataset_record_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Update dataset record properties
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
    ///     client.beta.observability.datasets.records.update_dataset_record_properties_v1observability_dataset_records_dataset_record_id_properties_put(&"dataset_record_id".to_string(), &UpdateDatasetRecordPropertiesRequest {
    ///         properties: HashMap::from([("key".to_string(), serde_json::json!("value"))])
    ///     }, None).await;
    /// }
    /// ```
    pub async fn update_dataset_record_properties_v1observability_dataset_records_dataset_record_id_properties_put(
        &self,
        dataset_record_id: &str,
        request: &UpdateDatasetRecordPropertiesRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "v1/observability/dataset-records/{}/properties",
                    dataset_record_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
