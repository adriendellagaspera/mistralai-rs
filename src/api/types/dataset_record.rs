pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasetRecord {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub dataset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub payload: DatasetRecordPayload,
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
    pub source: DatasetRecordSource,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl DatasetRecord {
    pub fn builder() -> DatasetRecordBuilder {
        <DatasetRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetRecordBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    dataset_id: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    payload: Option<DatasetRecordPayload>,
    properties: Option<HashMap<String, serde_json::Value>>,
    source: Option<DatasetRecordSource>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl DatasetRecordBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn dataset_id(mut self, value: impl Into<String>) -> Self {
        self.dataset_id = Some(value.into());
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

    pub fn payload(mut self, value: DatasetRecordPayload) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn properties(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn source(mut self, value: DatasetRecordSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](DatasetRecordBuilder::created_at)
    /// - [`dataset_id`](DatasetRecordBuilder::dataset_id)
    /// - [`id`](DatasetRecordBuilder::id)
    /// - [`payload`](DatasetRecordBuilder::payload)
    /// - [`properties`](DatasetRecordBuilder::properties)
    /// - [`source`](DatasetRecordBuilder::source)
    /// - [`updated_at`](DatasetRecordBuilder::updated_at)
    pub fn build(self) -> Result<DatasetRecord, BuildError> {
        Ok(DatasetRecord {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            dataset_id: self
                .dataset_id
                .ok_or_else(|| BuildError::missing_field("dataset_id"))?,
            deleted_at: self.deleted_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            payload: self
                .payload
                .ok_or_else(|| BuildError::missing_field("payload"))?,
            properties: self
                .properties
                .ok_or_else(|| BuildError::missing_field("properties"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
