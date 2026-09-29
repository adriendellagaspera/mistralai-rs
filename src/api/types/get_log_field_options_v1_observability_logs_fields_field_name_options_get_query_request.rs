pub use crate::prelude::*;

/// Query parameters for get_log_field_options_v1_observability_logs_fields__field_name__options_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLogFieldOptionsV1ObservabilityLogsFieldsFieldNameOptionsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<FixedOffset>>,
}

impl GetLogFieldOptionsV1ObservabilityLogsFieldsFieldNameOptionsGetQueryRequest {
    pub fn builder(
    ) -> GetLogFieldOptionsV1ObservabilityLogsFieldsFieldNameOptionsGetQueryRequestBuilder {
        <GetLogFieldOptionsV1ObservabilityLogsFieldsFieldNameOptionsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLogFieldOptionsV1ObservabilityLogsFieldsFieldNameOptionsGetQueryRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
}

impl GetLogFieldOptionsV1ObservabilityLogsFieldsFieldNameOptionsGetQueryRequestBuilder {
    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLogFieldOptionsV1ObservabilityLogsFieldsFieldNameOptionsGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<
        GetLogFieldOptionsV1ObservabilityLogsFieldsFieldNameOptionsGetQueryRequest,
        BuildError,
    > {
        Ok(
            GetLogFieldOptionsV1ObservabilityLogsFieldsFieldNameOptionsGetQueryRequest {
                from: self.from,
                to: self.to,
            },
        )
    }
}
