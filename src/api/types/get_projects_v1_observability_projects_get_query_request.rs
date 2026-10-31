pub use crate::prelude::*;

/// Query parameters for get_projects_v1_observability_projects_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetProjectsV1ObservabilityProjectsGetQueryRequest {
    /// Case-insensitive filter on project name/slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl GetProjectsV1ObservabilityProjectsGetQueryRequest {
    pub fn builder() -> GetProjectsV1ObservabilityProjectsGetQueryRequestBuilder {
        <GetProjectsV1ObservabilityProjectsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProjectsV1ObservabilityProjectsGetQueryRequestBuilder {
    q: Option<String>,
    page_size: Option<i64>,
    page: Option<i64>,
}

impl GetProjectsV1ObservabilityProjectsGetQueryRequestBuilder {
    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
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

    /// Consumes the builder and constructs a [`GetProjectsV1ObservabilityProjectsGetQueryRequest`].
    pub fn build(self) -> Result<GetProjectsV1ObservabilityProjectsGetQueryRequest, BuildError> {
        Ok(GetProjectsV1ObservabilityProjectsGetQueryRequest {
            q: self.q,
            page_size: self.page_size,
            page: self.page,
        })
    }
}
