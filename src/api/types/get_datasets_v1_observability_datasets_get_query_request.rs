pub use crate::prelude::*;

/// Query parameters for get_datasets_v1_observability_datasets_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDatasetsV1ObservabilityDatasetsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

impl GetDatasetsV1ObservabilityDatasetsGetQueryRequest {
    pub fn builder() -> GetDatasetsV1ObservabilityDatasetsGetQueryRequestBuilder {
        <GetDatasetsV1ObservabilityDatasetsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDatasetsV1ObservabilityDatasetsGetQueryRequestBuilder {
    page_size: Option<i64>,
    page: Option<i64>,
    q: Option<String>,
}

impl GetDatasetsV1ObservabilityDatasetsGetQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`GetDatasetsV1ObservabilityDatasetsGetQueryRequest`].
    pub fn build(self) -> Result<GetDatasetsV1ObservabilityDatasetsGetQueryRequest, BuildError> {
        Ok(GetDatasetsV1ObservabilityDatasetsGetQueryRequest {
            page_size: self.page_size,
            page: self.page,
            q: self.q,
        })
    }
}
