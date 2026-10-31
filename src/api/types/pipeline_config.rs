pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PipelineConfig {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    pub definition: PipelineConfigDefinition,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub pipeline_kind: PipelineKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<PipelineConfigSampling>,
    pub scope: PipelineConfigScope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scorer_slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scorer_version: Option<i64>,
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

impl PipelineConfig {
    pub fn builder() -> PipelineConfigBuilder {
        <PipelineConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineConfigBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    definition: Option<PipelineConfigDefinition>,
    deleted_at: Option<DateTime<FixedOffset>>,
    enabled: Option<bool>,
    id: Option<String>,
    name: Option<String>,
    pipeline_kind: Option<PipelineKind>,
    sampling: Option<PipelineConfigSampling>,
    scope: Option<PipelineConfigScope>,
    scorer_slug: Option<String>,
    scorer_version: Option<i64>,
    selectors: Option<Vec<PipelineConfigSelector>>,
    slug: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    workspace_id: Option<String>,
}

impl PipelineConfigBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn definition(mut self, value: PipelineConfigDefinition) -> Self {
        self.definition = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
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

    pub fn pipeline_kind(mut self, value: PipelineKind) -> Self {
        self.pipeline_kind = Some(value);
        self
    }

    pub fn sampling(mut self, value: PipelineConfigSampling) -> Self {
        self.sampling = Some(value);
        self
    }

    pub fn scope(mut self, value: PipelineConfigScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn scorer_slug(mut self, value: impl Into<String>) -> Self {
        self.scorer_slug = Some(value.into());
        self
    }

    pub fn scorer_version(mut self, value: i64) -> Self {
        self.scorer_version = Some(value);
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

    /// Consumes the builder and constructs a [`PipelineConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](PipelineConfigBuilder::created_at)
    /// - [`definition`](PipelineConfigBuilder::definition)
    /// - [`enabled`](PipelineConfigBuilder::enabled)
    /// - [`id`](PipelineConfigBuilder::id)
    /// - [`name`](PipelineConfigBuilder::name)
    /// - [`pipeline_kind`](PipelineConfigBuilder::pipeline_kind)
    /// - [`scope`](PipelineConfigBuilder::scope)
    /// - [`selectors`](PipelineConfigBuilder::selectors)
    /// - [`slug`](PipelineConfigBuilder::slug)
    /// - [`updated_at`](PipelineConfigBuilder::updated_at)
    /// - [`workspace_id`](PipelineConfigBuilder::workspace_id)
    pub fn build(self) -> Result<PipelineConfig, BuildError> {
        Ok(PipelineConfig {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            definition: self
                .definition
                .ok_or_else(|| BuildError::missing_field("definition"))?,
            deleted_at: self.deleted_at,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            pipeline_kind: self
                .pipeline_kind
                .ok_or_else(|| BuildError::missing_field("pipeline_kind"))?,
            sampling: self.sampling,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
            scorer_slug: self.scorer_slug,
            scorer_version: self.scorer_version,
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
