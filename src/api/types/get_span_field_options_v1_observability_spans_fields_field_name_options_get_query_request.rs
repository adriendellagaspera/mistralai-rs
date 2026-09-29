pub use crate::prelude::*;

/// Query parameters for get_span_field_options_v1_observability_spans_fields__field_name__options_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSpanFieldOptionsV1ObservabilitySpansFieldsFieldNameOptionsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<FixedOffset>>,
}

impl GetSpanFieldOptionsV1ObservabilitySpansFieldsFieldNameOptionsGetQueryRequest {
    pub fn builder(
    ) -> GetSpanFieldOptionsV1ObservabilitySpansFieldsFieldNameOptionsGetQueryRequestBuilder {
        <GetSpanFieldOptionsV1ObservabilitySpansFieldsFieldNameOptionsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSpanFieldOptionsV1ObservabilitySpansFieldsFieldNameOptionsGetQueryRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
}

impl GetSpanFieldOptionsV1ObservabilitySpansFieldsFieldNameOptionsGetQueryRequestBuilder {
    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSpanFieldOptionsV1ObservabilitySpansFieldsFieldNameOptionsGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<
        GetSpanFieldOptionsV1ObservabilitySpansFieldsFieldNameOptionsGetQueryRequest,
        BuildError,
    > {
        Ok(
            GetSpanFieldOptionsV1ObservabilitySpansFieldsFieldNameOptionsGetQueryRequest {
                from: self.from,
                to: self.to,
            },
        )
    }
}
