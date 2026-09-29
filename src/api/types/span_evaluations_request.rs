pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpanEvaluationsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_expression: Option<String>,
}

impl SpanEvaluationsRequest {
    pub fn builder() -> SpanEvaluationsRequestBuilder {
        <SpanEvaluationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpanEvaluationsRequestBuilder {
    search_expression: Option<String>,
}

impl SpanEvaluationsRequestBuilder {
    pub fn search_expression(mut self, value: impl Into<String>) -> Self {
        self.search_expression = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpanEvaluationsRequest`].
    pub fn build(self) -> Result<SpanEvaluationsRequest, BuildError> {
        Ok(SpanEvaluationsRequest {
            search_expression: self.search_expression,
        })
    }
}
