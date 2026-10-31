pub use crate::prelude::*;

/// Query parameters for list_optimization_trials_v1_observability_optimizations__optimization_id__trials_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOptimizationTrialsV1ObservabilityOptimizationsOptimizationIdTrialsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl ListOptimizationTrialsV1ObservabilityOptimizationsOptimizationIdTrialsGetQueryRequest {
    pub fn builder(
    ) -> ListOptimizationTrialsV1ObservabilityOptimizationsOptimizationIdTrialsGetQueryRequestBuilder
    {
        <ListOptimizationTrialsV1ObservabilityOptimizationsOptimizationIdTrialsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOptimizationTrialsV1ObservabilityOptimizationsOptimizationIdTrialsGetQueryRequestBuilder
{
    page_size: Option<i64>,
    page: Option<i64>,
}

impl ListOptimizationTrialsV1ObservabilityOptimizationsOptimizationIdTrialsGetQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOptimizationTrialsV1ObservabilityOptimizationsOptimizationIdTrialsGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<
        ListOptimizationTrialsV1ObservabilityOptimizationsOptimizationIdTrialsGetQueryRequest,
        BuildError,
    > {
        Ok(
            ListOptimizationTrialsV1ObservabilityOptimizationsOptimizationIdTrialsGetQueryRequest {
                page_size: self.page_size,
                page: self.page,
            },
        )
    }
}
