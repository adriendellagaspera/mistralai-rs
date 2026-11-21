pub use crate::prelude::*;

/// Query parameters for get_span_by_id
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSpanByIdQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<FixedOffset>>,
}

impl GetSpanByIdQueryRequest {
    pub fn builder() -> GetSpanByIdQueryRequestBuilder {
        <GetSpanByIdQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSpanByIdQueryRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
}

impl GetSpanByIdQueryRequestBuilder {
    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSpanByIdQueryRequest`].
    pub fn build(self) -> Result<GetSpanByIdQueryRequest, BuildError> {
        Ok(GetSpanByIdQueryRequest {
            from: self.from,
            to: self.to,
        })
    }
}
