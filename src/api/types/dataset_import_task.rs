pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DatasetImportTask {
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
    pub creator_id: String,
    #[serde(default)]
    pub dataset_id: String,
    #[serde(default)]
    pub workspace_id: String,
    pub status: BaseTaskStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DatasetImportTask {
    pub fn builder() -> DatasetImportTaskBuilder {
        <DatasetImportTaskBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetImportTaskBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    deleted_at: Option<DateTime<FixedOffset>>,
    creator_id: Option<String>,
    dataset_id: Option<String>,
    workspace_id: Option<String>,
    status: Option<BaseTaskStatus>,
    progress: Option<i64>,
    message: Option<String>,
}

impl DatasetImportTaskBuilder {
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

    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
        self
    }

    pub fn dataset_id(mut self, value: impl Into<String>) -> Self {
        self.dataset_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: BaseTaskStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn progress(mut self, value: i64) -> Self {
        self.progress = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DatasetImportTask`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DatasetImportTaskBuilder::id)
    /// - [`created_at`](DatasetImportTaskBuilder::created_at)
    /// - [`updated_at`](DatasetImportTaskBuilder::updated_at)
    /// - [`creator_id`](DatasetImportTaskBuilder::creator_id)
    /// - [`dataset_id`](DatasetImportTaskBuilder::dataset_id)
    /// - [`workspace_id`](DatasetImportTaskBuilder::workspace_id)
    /// - [`status`](DatasetImportTaskBuilder::status)
    pub fn build(self) -> Result<DatasetImportTask, BuildError> {
        Ok(DatasetImportTask {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            deleted_at: self.deleted_at,
            creator_id: self
                .creator_id
                .ok_or_else(|| BuildError::missing_field("creator_id"))?,
            dataset_id: self
                .dataset_id
                .ok_or_else(|| BuildError::missing_field("dataset_id"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            progress: self.progress,
            message: self.message,
        })
    }
}
