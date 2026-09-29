pub use crate::prelude::*;

/// Query parameters for get_trace_field_options_v1_observability_traces_fields__field_name__options_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTraceFieldOptionsV1ObservabilityTracesFieldsFieldNameOptionsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<FixedOffset>>,
}

impl GetTraceFieldOptionsV1ObservabilityTracesFieldsFieldNameOptionsGetQueryRequest {
    pub fn builder(
    ) -> GetTraceFieldOptionsV1ObservabilityTracesFieldsFieldNameOptionsGetQueryRequestBuilder {
        <GetTraceFieldOptionsV1ObservabilityTracesFieldsFieldNameOptionsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTraceFieldOptionsV1ObservabilityTracesFieldsFieldNameOptionsGetQueryRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
}

impl GetTraceFieldOptionsV1ObservabilityTracesFieldsFieldNameOptionsGetQueryRequestBuilder {
    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTraceFieldOptionsV1ObservabilityTracesFieldsFieldNameOptionsGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<
        GetTraceFieldOptionsV1ObservabilityTracesFieldsFieldNameOptionsGetQueryRequest,
        BuildError,
    > {
        Ok(
            GetTraceFieldOptionsV1ObservabilityTracesFieldsFieldNameOptionsGetQueryRequest {
                from: self.from,
                to: self.to,
            },
        )
    }
}
