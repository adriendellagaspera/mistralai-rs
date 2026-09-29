pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasetRecord {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub dataset_id: String,
    #[serde(default)]
    pub payload: DatasetRecordPayload,
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
    pub source: DatasetRecordSource,
}

impl DatasetRecord {
    pub fn builder() -> DatasetRecordBuilder {
        <DatasetRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetRecordBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    deleted_at: Option<DateTime<FixedOffset>>,
    dataset_id: Option<String>,
    payload: Option<DatasetRecordPayload>,
    properties: Option<HashMap<String, serde_json::Value>>,
    source: Option<DatasetRecordSource>,
}

impl DatasetRecordBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn dataset_id(mut self, value: impl Into<String>) -> Self {
        self.dataset_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`DatasetRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DatasetRecordBuilder::id)
    /// - [`created_at`](DatasetRecordBuilder::created_at)
    /// - [`updated_at`](DatasetRecordBuilder::updated_at)
    /// - [`dataset_id`](DatasetRecordBuilder::dataset_id)
    /// - [`payload`](DatasetRecordBuilder::payload)
    /// - [`properties`](DatasetRecordBuilder::properties)
    /// - [`source`](DatasetRecordBuilder::source)
    pub fn build(self) -> Result<DatasetRecord, BuildError> {
        Ok(DatasetRecord {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            deleted_at: self.deleted_at,
            dataset_id: self
                .dataset_id
                .ok_or_else(|| BuildError::missing_field("dataset_id"))?,
            payload: self
                .payload
                .ok_or_else(|| BuildError::missing_field("payload"))?,
            properties: self
                .properties
                .ok_or_else(|| BuildError::missing_field("properties"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
