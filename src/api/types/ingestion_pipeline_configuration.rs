pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IngestionPipelineConfiguration {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub author_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub modified_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_run_time: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub last_run_chunks_count: i64,
    #[serde(default)]
    pub total_chunks_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_composition: Option<HashMap<String, Option<String>>>,
}

impl IngestionPipelineConfiguration {
    pub fn builder() -> IngestionPipelineConfigurationBuilder {
        <IngestionPipelineConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngestionPipelineConfigurationBuilder {
    id: Option<String>,
    author_id: Option<String>,
    name: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    modified_at: Option<DateTime<FixedOffset>>,
    last_run_time: Option<DateTime<FixedOffset>>,
    last_run_chunks_count: Option<i64>,
    total_chunks_count: Option<i64>,
    pipeline_composition: Option<HashMap<String, Option<String>>>,
}

impl IngestionPipelineConfigurationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn author_id(mut self, value: impl Into<String>) -> Self {
        self.author_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn last_run_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_run_time = Some(value);
        self
    }

    pub fn last_run_chunks_count(mut self, value: i64) -> Self {
        self.last_run_chunks_count = Some(value);
        self
    }

    pub fn total_chunks_count(mut self, value: i64) -> Self {
        self.total_chunks_count = Some(value);
        self
    }

    pub fn pipeline_composition(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.pipeline_composition = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IngestionPipelineConfiguration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](IngestionPipelineConfigurationBuilder::id)
    /// - [`author_id`](IngestionPipelineConfigurationBuilder::author_id)
    /// - [`name`](IngestionPipelineConfigurationBuilder::name)
    /// - [`created_at`](IngestionPipelineConfigurationBuilder::created_at)
    /// - [`modified_at`](IngestionPipelineConfigurationBuilder::modified_at)
    /// - [`last_run_chunks_count`](IngestionPipelineConfigurationBuilder::last_run_chunks_count)
    /// - [`total_chunks_count`](IngestionPipelineConfigurationBuilder::total_chunks_count)
    pub fn build(self) -> Result<IngestionPipelineConfiguration, BuildError> {
        Ok(IngestionPipelineConfiguration {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            author_id: self
                .author_id
                .ok_or_else(|| BuildError::missing_field("author_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            last_run_time: self.last_run_time,
            last_run_chunks_count: self
                .last_run_chunks_count
                .ok_or_else(|| BuildError::missing_field("last_run_chunks_count"))?,
            total_chunks_count: self
                .total_chunks_count
                .ok_or_else(|| BuildError::missing_field("total_chunks_count"))?,
            pipeline_composition: self.pipeline_composition,
        })
    }
}
