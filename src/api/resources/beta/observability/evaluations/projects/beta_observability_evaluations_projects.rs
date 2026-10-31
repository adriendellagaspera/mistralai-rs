use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ProjectsClient {
    pub http_client: HttpClient,
}

impl ProjectsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List projects
    ///
    /// # Arguments
    ///
    /// * `q` - Case-insensitive filter on project name/slug.
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
    ///         .observability
    ///         .evaluations
    ///         .projects
    ///         .get_projects_v1observability_projects_get(
    ///             &GetProjectsV1ObservabilityProjectsGetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_projects_v1observability_projects_get(
        &self,
        request: &GetProjectsV1ObservabilityProjectsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Projects, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/observability/projects",
                None,
                QueryBuilder::new()
                    .serialize("q", request.q.clone())
                    .int("page_size", request.page_size.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get project by slug
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
    ///         .observability
    ///         .evaluations
    ///         .projects
    ///         .get_project_v1observability_projects_project_slug_get(&"project_slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_project_v1observability_projects_project_slug_get(
        &self,
        project_slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<Project, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/observability/projects/{}", project_slug),
                None,
                None,
                options,
            )
            .await
    }
}
