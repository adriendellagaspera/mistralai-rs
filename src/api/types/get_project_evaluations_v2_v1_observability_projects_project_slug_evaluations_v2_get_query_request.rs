pub use crate::prelude::*;

/// Query parameters for get_project_evaluations_v2_v1_observability_projects__project_slug__evaluations_v2_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetProjectEvaluationsV2V1ObservabilityProjectsProjectSlugEvaluationsV2GetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl GetProjectEvaluationsV2V1ObservabilityProjectsProjectSlugEvaluationsV2GetQueryRequest {
    pub fn builder(
    ) -> GetProjectEvaluationsV2V1ObservabilityProjectsProjectSlugEvaluationsV2GetQueryRequestBuilder
    {
        <GetProjectEvaluationsV2V1ObservabilityProjectsProjectSlugEvaluationsV2GetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProjectEvaluationsV2V1ObservabilityProjectsProjectSlugEvaluationsV2GetQueryRequestBuilder
{
    page_size: Option<i64>,
    page: Option<i64>,
}

impl GetProjectEvaluationsV2V1ObservabilityProjectsProjectSlugEvaluationsV2GetQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetProjectEvaluationsV2V1ObservabilityProjectsProjectSlugEvaluationsV2GetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<
        GetProjectEvaluationsV2V1ObservabilityProjectsProjectSlugEvaluationsV2GetQueryRequest,
        BuildError,
    > {
        Ok(
            GetProjectEvaluationsV2V1ObservabilityProjectsProjectSlugEvaluationsV2GetQueryRequest {
                page_size: self.page_size,
                page: self.page,
            },
        )
    }
}
