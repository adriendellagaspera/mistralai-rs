pub use crate::prelude::*;

/// Pipeline response schema. Pipelines are hard-deleted — no deleted_at field.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Pipeline {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub pipeline_configs: Vec<PipelineConfig>,
    #[serde(default)]
    pub selectors: Vec<PipelineConfigSelector>,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub workspace_id: String,
}

impl Pipeline {
    pub fn builder() -> PipelineBuilder {
        <PipelineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    creator_id: Option<String>,
    description: Option<String>,
    enabled: Option<bool>,
    id: Option<String>,
    name: Option<String>,
    pipeline_configs: Option<Vec<PipelineConfig>>,
    selectors: Option<Vec<PipelineConfigSelector>>,
    slug: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    workspace_id: Option<String>,
}

impl PipelineBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
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

    pub fn pipeline_configs(mut self, value: Vec<PipelineConfig>) -> Self {
        self.pipeline_configs = Some(value);
        self
    }

    pub fn selectors(mut self, value: Vec<PipelineConfigSelector>) -> Self {
        self.selectors = Some(value);
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
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

    /// Consumes the builder and constructs a [`Pipeline`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](PipelineBuilder::created_at)
    /// - [`enabled`](PipelineBuilder::enabled)
    /// - [`id`](PipelineBuilder::id)
    /// - [`name`](PipelineBuilder::name)
    /// - [`pipeline_configs`](PipelineBuilder::pipeline_configs)
    /// - [`selectors`](PipelineBuilder::selectors)
    /// - [`slug`](PipelineBuilder::slug)
    /// - [`updated_at`](PipelineBuilder::updated_at)
    /// - [`workspace_id`](PipelineBuilder::workspace_id)
    pub fn build(self) -> Result<Pipeline, BuildError> {
        Ok(Pipeline {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            creator_id: self.creator_id,
            description: self.description,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            pipeline_configs: self
                .pipeline_configs
                .ok_or_else(|| BuildError::missing_field("pipeline_configs"))?,
            selectors: self
                .selectors
                .ok_or_else(|| BuildError::missing_field("selectors"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
