pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Scorer {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub kind: ScorerKind,
    pub latest_version: ScorerVersion,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl Scorer {
    pub fn builder() -> ScorerBuilder {
        <ScorerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScorerBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    kind: Option<ScorerKind>,
    latest_version: Option<ScorerVersion>,
    name: Option<String>,
    slug: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ScorerBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ScorerKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn latest_version(mut self, value: ScorerVersion) -> Self {
        self.latest_version = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    /// Consumes the builder and constructs a [`Scorer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ScorerBuilder::created_at)
    /// - [`kind`](ScorerBuilder::kind)
    /// - [`latest_version`](ScorerBuilder::latest_version)
    /// - [`name`](ScorerBuilder::name)
    /// - [`slug`](ScorerBuilder::slug)
    /// - [`updated_at`](ScorerBuilder::updated_at)
    pub fn build(self) -> Result<Scorer, BuildError> {
        Ok(Scorer {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            latest_version: self
                .latest_version
                .ok_or_else(|| BuildError::missing_field("latest_version"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
