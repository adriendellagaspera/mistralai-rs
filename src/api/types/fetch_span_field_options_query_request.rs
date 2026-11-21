pub use crate::prelude::*;

/// Query parameters for fetch_span_field_options
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FetchSpanFieldOptionsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<FixedOffset>>,
}

impl FetchSpanFieldOptionsQueryRequest {
    pub fn builder() -> FetchSpanFieldOptionsQueryRequestBuilder {
        <FetchSpanFieldOptionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FetchSpanFieldOptionsQueryRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
}

impl FetchSpanFieldOptionsQueryRequestBuilder {
    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FetchSpanFieldOptionsQueryRequest`].
    pub fn build(self) -> Result<FetchSpanFieldOptionsQueryRequest, BuildError> {
        Ok(FetchSpanFieldOptionsQueryRequest {
            from: self.from,
            to: self.to,
        })
    }
}
