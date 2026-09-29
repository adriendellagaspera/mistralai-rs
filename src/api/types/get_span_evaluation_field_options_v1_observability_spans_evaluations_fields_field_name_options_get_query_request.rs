pub use crate::prelude::*;

/// Query parameters for get_span_evaluation_field_options_v1_observability_spans_evaluations_fields__field_name__options_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSpanEvaluationFieldOptionsV1ObservabilitySpansEvaluationsFieldsFieldNameOptionsGetQueryRequest
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<FixedOffset>>,
}

impl GetSpanEvaluationFieldOptionsV1ObservabilitySpansEvaluationsFieldsFieldNameOptionsGetQueryRequest {
    pub fn builder() -> GetSpanEvaluationFieldOptionsV1ObservabilitySpansEvaluationsFieldsFieldNameOptionsGetQueryRequestBuilder {
        <GetSpanEvaluationFieldOptionsV1ObservabilitySpansEvaluationsFieldsFieldNameOptionsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSpanEvaluationFieldOptionsV1ObservabilitySpansEvaluationsFieldsFieldNameOptionsGetQueryRequestBuilder
{
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
}

impl GetSpanEvaluationFieldOptionsV1ObservabilitySpansEvaluationsFieldsFieldNameOptionsGetQueryRequestBuilder {
    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSpanEvaluationFieldOptionsV1ObservabilitySpansEvaluationsFieldsFieldNameOptionsGetQueryRequest`].
    pub fn build(self) -> Result<GetSpanEvaluationFieldOptionsV1ObservabilitySpansEvaluationsFieldsFieldNameOptionsGetQueryRequest, BuildError> {
        Ok(GetSpanEvaluationFieldOptionsV1ObservabilitySpansEvaluationsFieldsFieldNameOptionsGetQueryRequest {
            from: self.from,
            to: self.to,
        })
    }
}
