pub use crate::prelude::*;

/// Request for search_span_evaluations (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchSpanEvaluationsRequest {
    #[serde(skip)]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip)]
    pub to: Option<DateTime<FixedOffset>>,
    #[serde(skip)]
    pub page_size: Option<i64>,
    #[serde(skip)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub body: SpanEvaluationsRequest,
}

impl SearchSpanEvaluationsRequest {
    pub fn builder() -> SearchSpanEvaluationsRequestBuilder {
        <SearchSpanEvaluationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchSpanEvaluationsRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    page_size: Option<i64>,
    cursor: Option<String>,
    body: Option<SpanEvaluationsRequest>,
}

impl SearchSpanEvaluationsRequestBuilder {
    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn body(mut self, value: SpanEvaluationsRequest) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchSpanEvaluationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](SearchSpanEvaluationsRequestBuilder::body)
    pub fn build(self) -> Result<SearchSpanEvaluationsRequest, BuildError> {
        Ok(SearchSpanEvaluationsRequest {
            from: self.from,
            to: self.to,
            page_size: self.page_size,
            cursor: self.cursor,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
