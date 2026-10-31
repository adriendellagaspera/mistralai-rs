pub use crate::prelude::*;

/// Query parameters for list_pipelines_v1_observability_pipelines_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPipelinesV1ObservabilityPipelinesGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

impl ListPipelinesV1ObservabilityPipelinesGetQueryRequest {
    pub fn builder() -> ListPipelinesV1ObservabilityPipelinesGetQueryRequestBuilder {
        <ListPipelinesV1ObservabilityPipelinesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPipelinesV1ObservabilityPipelinesGetQueryRequestBuilder {
    enabled: Option<bool>,
    page_size: Option<i64>,
    page: Option<i64>,
    q: Option<String>,
}

impl ListPipelinesV1ObservabilityPipelinesGetQueryRequestBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
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

    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPipelinesV1ObservabilityPipelinesGetQueryRequest`].
    pub fn build(self) -> Result<ListPipelinesV1ObservabilityPipelinesGetQueryRequest, BuildError> {
        Ok(ListPipelinesV1ObservabilityPipelinesGetQueryRequest {
            enabled: self.enabled,
            page_size: self.page_size,
            page: self.page,
            q: self.q,
        })
    }
}
