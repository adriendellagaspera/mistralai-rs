pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Library {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_size: Option<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_user_members_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_workspace_members_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_description: Option<String>,
    /// Generated Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_name: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub nb_documents: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_sharing_role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default)]
    pub owner_type: String,
    #[serde(default)]
    pub total_size: i64,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl Library {
    pub fn builder() -> LibraryBuilder {
        <LibraryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LibraryBuilder {
    chunk_size: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    emoji: Option<String>,
    explicit_user_members_count: Option<i64>,
    explicit_workspace_members_count: Option<i64>,
    generated_description: Option<String>,
    generated_name: Option<String>,
    id: Option<String>,
    name: Option<String>,
    nb_documents: Option<i64>,
    org_sharing_role: Option<String>,
    owner_id: Option<String>,
    owner_type: Option<String>,
    total_size: Option<i64>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl LibraryBuilder {
    pub fn chunk_size(mut self, value: i64) -> Self {
        self.chunk_size = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn emoji(mut self, value: impl Into<String>) -> Self {
        self.emoji = Some(value.into());
        self
    }

    pub fn explicit_user_members_count(mut self, value: i64) -> Self {
        self.explicit_user_members_count = Some(value);
        self
    }

    pub fn explicit_workspace_members_count(mut self, value: i64) -> Self {
        self.explicit_workspace_members_count = Some(value);
        self
    }

    pub fn generated_description(mut self, value: impl Into<String>) -> Self {
        self.generated_description = Some(value.into());
        self
    }

    pub fn generated_name(mut self, value: impl Into<String>) -> Self {
        self.generated_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn nb_documents(mut self, value: i64) -> Self {
        self.nb_documents = Some(value);
        self
    }

    pub fn org_sharing_role(mut self, value: impl Into<String>) -> Self {
        self.org_sharing_role = Some(value.into());
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    pub fn owner_type(mut self, value: impl Into<String>) -> Self {
        self.owner_type = Some(value.into());
        self
    }

    pub fn total_size(mut self, value: i64) -> Self {
        self.total_size = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Library`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](LibraryBuilder::created_at)
    /// - [`id`](LibraryBuilder::id)
    /// - [`name`](LibraryBuilder::name)
    /// - [`nb_documents`](LibraryBuilder::nb_documents)
    /// - [`owner_type`](LibraryBuilder::owner_type)
    /// - [`total_size`](LibraryBuilder::total_size)
    /// - [`updated_at`](LibraryBuilder::updated_at)
    pub fn build(self) -> Result<Library, BuildError> {
        Ok(Library {
            chunk_size: self.chunk_size,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            emoji: self.emoji,
            explicit_user_members_count: self.explicit_user_members_count,
            explicit_workspace_members_count: self.explicit_workspace_members_count,
            generated_description: self.generated_description,
            generated_name: self.generated_name,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            nb_documents: self
                .nb_documents
                .ok_or_else(|| BuildError::missing_field("nb_documents"))?,
            org_sharing_role: self.org_sharing_role,
            owner_id: self.owner_id,
            owner_type: self
                .owner_type
                .ok_or_else(|| BuildError::missing_field("owner_type"))?,
            total_size: self
                .total_size
                .ok_or_else(|| BuildError::missing_field("total_size"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
