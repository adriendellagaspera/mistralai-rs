pub use crate::prelude::*;

/// Query parameters for get_judges_v1_observability_judges_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetJudgesV1ObservabilityJudgesGetQueryRequest {
    /// Filter by judge output types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<Vec<JudgeOutputType>>,
    /// Filter by model names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_filter: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

impl GetJudgesV1ObservabilityJudgesGetQueryRequest {
    pub fn builder() -> GetJudgesV1ObservabilityJudgesGetQueryRequestBuilder {
        <GetJudgesV1ObservabilityJudgesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetJudgesV1ObservabilityJudgesGetQueryRequestBuilder {
    type_filter: Option<Vec<JudgeOutputType>>,
    model_filter: Option<Vec<String>>,
    page_size: Option<i64>,
    page: Option<i64>,
    q: Option<String>,
}

impl GetJudgesV1ObservabilityJudgesGetQueryRequestBuilder {
    pub fn type_filter(mut self, value: Vec<JudgeOutputType>) -> Self {
        self.type_filter = Some(value);
        self
    }

    pub fn model_filter(mut self, value: Vec<String>) -> Self {
        self.model_filter = Some(value);
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

    /// Consumes the builder and constructs a [`GetJudgesV1ObservabilityJudgesGetQueryRequest`].
    pub fn build(self) -> Result<GetJudgesV1ObservabilityJudgesGetQueryRequest, BuildError> {
        Ok(GetJudgesV1ObservabilityJudgesGetQueryRequest {
            type_filter: self.type_filter,
            model_filter: self.model_filter,
            page_size: self.page_size,
            page: self.page,
            q: self.q,
        })
    }
}
