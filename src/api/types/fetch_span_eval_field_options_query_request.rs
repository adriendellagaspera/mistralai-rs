pub use crate::prelude::*;

/// Query parameters for fetch_span_eval_field_options
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FetchSpanEvalFieldOptionsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<FixedOffset>>,
}

impl FetchSpanEvalFieldOptionsQueryRequest {
    pub fn builder() -> FetchSpanEvalFieldOptionsQueryRequestBuilder {
        <FetchSpanEvalFieldOptionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FetchSpanEvalFieldOptionsQueryRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
}

impl FetchSpanEvalFieldOptionsQueryRequestBuilder {
    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FetchSpanEvalFieldOptionsQueryRequest`].
    pub fn build(self) -> Result<FetchSpanEvalFieldOptionsQueryRequest, BuildError> {
        Ok(FetchSpanEvalFieldOptionsQueryRequest {
            from: self.from,
            to: self.to,
        })
    }
}
