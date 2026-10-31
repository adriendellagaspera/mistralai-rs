pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvaluationV2 {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub creator_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    #[serde(rename = "metadata_")]
    #[serde(default)]
    pub metadata: HashMap<String, JsonValue>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub workspace_id: String,
}

impl EvaluationV2 {
    pub fn builder() -> EvaluationV2Builder {
        <EvaluationV2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluationV2Builder {
    created_at: Option<DateTime<FixedOffset>>,
    creator_id: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    id: Option<String>,
    is_favorite: Option<bool>,
    metadata: Option<HashMap<String, JsonValue>>,
    name: Option<String>,
    project_id: Option<String>,
    slug: Option<String>,
    tags: Option<Vec<String>>,
    updated_at: Option<DateTime<FixedOffset>>,
    workspace_id: Option<String>,
}

impl EvaluationV2Builder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, JsonValue>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

    /// Consumes the builder and constructs a [`EvaluationV2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](EvaluationV2Builder::created_at)
    /// - [`creator_id`](EvaluationV2Builder::creator_id)
    /// - [`id`](EvaluationV2Builder::id)
    /// - [`metadata`](EvaluationV2Builder::metadata)
    /// - [`name`](EvaluationV2Builder::name)
    /// - [`project_id`](EvaluationV2Builder::project_id)
    /// - [`slug`](EvaluationV2Builder::slug)
    /// - [`tags`](EvaluationV2Builder::tags)
    /// - [`updated_at`](EvaluationV2Builder::updated_at)
    /// - [`workspace_id`](EvaluationV2Builder::workspace_id)
    pub fn build(self) -> Result<EvaluationV2, BuildError> {
        Ok(EvaluationV2 {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            creator_id: self
                .creator_id
                .ok_or_else(|| BuildError::missing_field("creator_id"))?,
            deleted_at: self.deleted_at,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_favorite: self.is_favorite,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            project_id: self
                .project_id
                .ok_or_else(|| BuildError::missing_field("project_id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
