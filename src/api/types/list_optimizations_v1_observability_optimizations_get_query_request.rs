pub use crate::prelude::*;

/// Query parameters for list_optimizations_v1_observability_optimizations_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOptimizationsV1ObservabilityOptimizationsGetQueryRequest {
    /// Filter by project slug
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Filter by evaluation slug
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation: Option<String>,
    /// Filter by lifecycle status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by algorithm
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// Filter by tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl ListOptimizationsV1ObservabilityOptimizationsGetQueryRequest {
    pub fn builder() -> ListOptimizationsV1ObservabilityOptimizationsGetQueryRequestBuilder {
        <ListOptimizationsV1ObservabilityOptimizationsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOptimizationsV1ObservabilityOptimizationsGetQueryRequestBuilder {
    project: Option<String>,
    evaluation: Option<String>,
    status: Option<String>,
    algorithm: Option<String>,
    tags: Option<Vec<String>>,
    page_size: Option<i64>,
    page: Option<i64>,
}

impl ListOptimizationsV1ObservabilityOptimizationsGetQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    pub fn evaluation(mut self, value: impl Into<String>) -> Self {
        self.evaluation = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn algorithm(mut self, value: impl Into<String>) -> Self {
        self.algorithm = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOptimizationsV1ObservabilityOptimizationsGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<ListOptimizationsV1ObservabilityOptimizationsGetQueryRequest, BuildError> {
        Ok(
            ListOptimizationsV1ObservabilityOptimizationsGetQueryRequest {
                project: self.project,
                evaluation: self.evaluation,
                status: self.status,
                algorithm: self.algorithm,
                tags: self.tags,
                page_size: self.page_size,
                page: self.page,
            },
        )
    }
}
