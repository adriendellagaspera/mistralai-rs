pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvaluationRunScoreV2 {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "metadata_")]
    #[serde(default)]
    pub metadata: HashMap<String, JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
}

impl EvaluationRunScoreV2 {
    pub fn builder() -> EvaluationRunScoreV2Builder {
        <EvaluationRunScoreV2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluationRunScoreV2Builder {
    created_at: Option<DateTime<FixedOffset>>,
    deleted_at: Option<DateTime<FixedOffset>>,
    error: Option<String>,
    id: Option<String>,
    metadata: Option<HashMap<String, JsonValue>>,
    rationale: Option<String>,
    status: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    value: Option<JsonValue>,
}

impl EvaluationRunScoreV2Builder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, JsonValue>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn rationale(mut self, value: impl Into<String>) -> Self {
        self.rationale = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn value(mut self, value: JsonValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvaluationRunScoreV2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](EvaluationRunScoreV2Builder::created_at)
    /// - [`id`](EvaluationRunScoreV2Builder::id)
    /// - [`metadata`](EvaluationRunScoreV2Builder::metadata)
    /// - [`status`](EvaluationRunScoreV2Builder::status)
    /// - [`updated_at`](EvaluationRunScoreV2Builder::updated_at)
    pub fn build(self) -> Result<EvaluationRunScoreV2, BuildError> {
        Ok(EvaluationRunScoreV2 {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            error: self.error,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            rationale: self.rationale,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            value: self.value,
        })
    }
}
