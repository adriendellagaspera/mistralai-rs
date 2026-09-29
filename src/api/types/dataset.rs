pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Dataset {
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
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub owner_id: String,
    #[serde(default)]
    pub workspace_id: String,
}

impl Dataset {
    pub fn builder() -> DatasetBuilder {
        <DatasetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetBuilder {
    id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    deleted_at: Option<DateTime<FixedOffset>>,
    name: Option<String>,
    description: Option<String>,
    owner_id: Option<String>,
    workspace_id: Option<String>,
}

impl DatasetBuilder {
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

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Dataset`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DatasetBuilder::id)
    /// - [`created_at`](DatasetBuilder::created_at)
    /// - [`updated_at`](DatasetBuilder::updated_at)
    /// - [`name`](DatasetBuilder::name)
    /// - [`description`](DatasetBuilder::description)
    /// - [`owner_id`](DatasetBuilder::owner_id)
    /// - [`workspace_id`](DatasetBuilder::workspace_id)
    pub fn build(self) -> Result<Dataset, BuildError> {
        Ok(Dataset {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            deleted_at: self.deleted_at,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            owner_id: self
                .owner_id
                .ok_or_else(|| BuildError::missing_field("owner_id"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
