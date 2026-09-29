use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BatchClient {
    pub http_client: HttpClient,
}

impl BatchClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get a list of batch jobs for your organization and user.
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
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .batch
    ///         .jobs_api_routes_batch_get_batch_jobs(
    ///             &JobsAPIRoutesBatchGetBatchJobsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn jobs_api_routes_batch_get_batch_jobs(
        &self,
        request: &JobsApiRoutesBatchGetBatchJobsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListBatchJobsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/batch/jobs",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("model", request.model.clone())
                    .serialize("agent_id", request.agent_id.clone())
                    .serialize("metadata", request.metadata.clone())
                    .serialize("created_after", request.created_after.clone())
                    .bool("created_by_me", request.created_by_me.clone())
                    .serialize("status", request.status.clone())
                    .serialize("order_by", request.order_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new batch job, it will be queued for processing.
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
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .batch
    ///         .jobs_api_routes_batch_create_batch_job(
    ///             &CreateBatchJobRequest {
    ///                 endpoint: APIEndpoint::V1ChatCompletions,
    ///                 agent_id: None,
    ///                 input_files: None,
    ///                 metadata: None,
    ///                 model: None,
    ///                 requests: None,
    ///                 timeout_hours: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn jobs_api_routes_batch_create_batch_job(
        &self,
        request: &CreateBatchJobRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchJob, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/batch/jobs",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a batch job details by its UUID.
    ///
    /// Args:
    /// inline: If True, return results inline in the response.
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
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .batch
    ///         .jobs_api_routes_batch_get_batch_job(
    ///             &"job_id".to_string(),
    ///             &JobsAPIRoutesBatchGetBatchJobQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn jobs_api_routes_batch_get_batch_job(
        &self,
        job_id: &str,
        request: &JobsApiRoutesBatchGetBatchJobQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchJob, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/batch/jobs/{}", job_id),
                None,
                QueryBuilder::new()
                    .serialize("inline", request.inline.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Request the deletion of a batch job.
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
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .batch
    ///         .jobs_api_routes_batch_delete_batch_job(&"job_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn jobs_api_routes_batch_delete_batch_job(
        &self,
        job_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteBatchJobResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/batch/jobs/{}", job_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Request the cancellation of a batch job.
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
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .batch
    ///         .jobs_api_routes_batch_cancel_batch_job(&"job_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn jobs_api_routes_batch_cancel_batch_job(
        &self,
        job_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BatchJob, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/batch/jobs/{}/cancel", job_id),
                None,
                None,
                options,
            )
            .await
    }
}
