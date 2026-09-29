pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DatasetImportTask {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub creator_id: String,
    #[serde(default)]
    pub dataset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    pub status: BaseTaskStatus,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub workspace_id: String,
}

impl DatasetImportTask {
    pub fn builder() -> DatasetImportTaskBuilder {
        <DatasetImportTaskBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetImportTaskBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    creator_id: Option<String>,
    dataset_id: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    message: Option<String>,
    progress: Option<i64>,
    status: Option<BaseTaskStatus>,
    updated_at: Option<DateTime<FixedOffset>>,
    workspace_id: Option<String>,
}

impl DatasetImportTaskBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn progress(mut self, value: i64) -> Self {
        self.progress = Some(value);
        self
    }

    pub fn status(mut self, value: BaseTaskStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DatasetImportTask`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](DatasetImportTaskBuilder::created_at)
    /// - [`creator_id`](DatasetImportTaskBuilder::creator_id)
    /// - [`dataset_id`](DatasetImportTaskBuilder::dataset_id)
    /// - [`id`](DatasetImportTaskBuilder::id)
    /// - [`status`](DatasetImportTaskBuilder::status)
    /// - [`updated_at`](DatasetImportTaskBuilder::updated_at)
    /// - [`workspace_id`](DatasetImportTaskBuilder::workspace_id)
    pub fn build(self) -> Result<DatasetImportTask, BuildError> {
        Ok(DatasetImportTask {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            creator_id: self
                .creator_id
                .ok_or_else(|| BuildError::missing_field("creator_id"))?,
            dataset_id: self
                .dataset_id
                .ok_or_else(|| BuildError::missing_field("dataset_id"))?,
            deleted_at: self.deleted_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            message: self.message,
            progress: self.progress,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
