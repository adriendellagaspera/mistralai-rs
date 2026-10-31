pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvaluationRunRecordV2Response {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_record_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub id: String,
    pub input: JsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_record: Option<EvaluationRunOutputRecordV2>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl EvaluationRunRecordV2Response {
    pub fn builder() -> EvaluationRunRecordV2ResponseBuilder {
        <EvaluationRunRecordV2ResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluationRunRecordV2ResponseBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    dataset_record_id: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    input: Option<JsonValue>,
    output_record: Option<EvaluationRunOutputRecordV2>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl EvaluationRunRecordV2ResponseBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn dataset_record_id(mut self, value: impl Into<String>) -> Self {
        self.dataset_record_id = Some(value.into());
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn input(mut self, value: JsonValue) -> Self {
        self.input = Some(value);
        self
    }

    pub fn output_record(mut self, value: EvaluationRunOutputRecordV2) -> Self {
        self.output_record = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvaluationRunRecordV2Response`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](EvaluationRunRecordV2ResponseBuilder::created_at)
    /// - [`id`](EvaluationRunRecordV2ResponseBuilder::id)
    /// - [`input`](EvaluationRunRecordV2ResponseBuilder::input)
    /// - [`updated_at`](EvaluationRunRecordV2ResponseBuilder::updated_at)
    pub fn build(self) -> Result<EvaluationRunRecordV2Response, BuildError> {
        Ok(EvaluationRunRecordV2Response {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            dataset_record_id: self.dataset_record_id,
            deleted_at: self.deleted_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            output_record: self.output_record,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
