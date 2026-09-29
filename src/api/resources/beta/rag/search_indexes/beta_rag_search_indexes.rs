use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SearchIndexesClient {
    pub http_client: HttpClient,
}

impl SearchIndexesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Fetch all indexes available to a user
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
    ///         .rag
    ///         .search_indexes
    ///         .get_deployment_summaries_v1rag_deployments_get(None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_deployment_summaries_v1rag_deployments_get(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetDeploymentSummariesResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/rag/deployments", None, None, options)
            .await
    }

    /// Register (or re-register) a search index
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
    ///         .rag
    ///         .search_indexes
    ///         .register_deployment_v1rag_deployments_put(
    ///             &RegisterDeploymentRequestDeployment {
    ///                 name: "name".to_string(),
    ///                 deployment: RegisterDeploymentRequestDeploymentDeployment::Vespa {
    ///                     data: RegisterDeploymentRequestVespaDeployment {
    ///                         vespa_version: "vespa_version".to_string(),
    ///                         indexes: vec![RegisterDeploymentRequestVespaIndex {
    ///                             name: "name".to_string(),
    ///                             fields: vec![RegisterDeploymentRequestVespaField {
    ///                                 name: "name".to_string(),
    ///                                 r#type: SchemaFieldDataType::Int,
    ///                                 storage: SchemaFieldStorage::InMemory,
    ///                                 ranking: SchemaFieldRankingType::Count,
    ///                                 index_type: None,
    ///                                 multidimensional: true,
    ///                             }],
    ///                             sd: "sd".to_string(),
    ///                             ..Default::default()
    ///                         }],
    ///                         query_url: "query_url".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                 },
    ///                 status: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn register_deployment_v1rag_deployments_put(
        &self,
        request: &RegisterDeploymentRequestDeployment,
        options: Option<RequestOptions>,
    ) -> Result<RegisterSearchIndexResponseIndex, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                "v1/rag/deployments",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete all information about a deployment
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
    ///         .rag
    ///         .search_indexes
    ///         .unregister_deployment_v1rag_deployments_deployment_id_delete(
    ///             &"deployment_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn unregister_deployment_v1rag_deployments_deployment_id_delete(
        &self,
        deployment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/rag/deployments/{}", deployment_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the metrics for a given index
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
    ///     client.beta.rag.search_indexes.update_index_metrics_v1rag_deployments_deployment_id_metrics_put(&"deployment_id".to_string(), &UpdateIndexMetricsV1RagDeploymentsDeploymentIDMetricsPutSearchIndexesRequestBody::Online {
    ///         data: UpdateMetricsRequestDeploymentMetricsOnline {
    ///             document_count: 1,
    ///             index_metrics: vec![UpdateMetricsRequestIndexMetrics {
    ///                 name: "name".to_string(),
    ///                 document_count: 1,
    ///                 ..Default::default()
    ///             }],
    ///             ..Default::default()
    ///         }
    ///     }, None).await;
    /// }
    /// ```
    pub async fn update_index_metrics_v1rag_deployments_deployment_id_metrics_put(
        &self,
        deployment_id: &str,
        request: &UpdateIndexMetricsV1RagDeploymentsDeploymentIdMetricsPutSearchIndexesRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/rag/deployments/{}/metrics", deployment_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
