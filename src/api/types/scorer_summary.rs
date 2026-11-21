pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScorerSummary {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub creator_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<PrincipalType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub is_ootb: bool,
    pub kind: ScorerKind,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ScorerSummary {
    pub fn builder() -> ScorerSummaryBuilder {
        <ScorerSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScorerSummaryBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    creator_id: Option<String>,
    creator_type: Option<PrincipalType>,
    description: Option<String>,
    is_ootb: Option<bool>,
    kind: Option<ScorerKind>,
    name: Option<String>,
    slug: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ScorerSummaryBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
        self
    }

    pub fn creator_type(mut self, value: PrincipalType) -> Self {
        self.creator_type = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn is_ootb(mut self, value: bool) -> Self {
        self.is_ootb = Some(value);
        self
    }

    pub fn kind(mut self, value: ScorerKind) -> Self {
        self.kind = Some(value);
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

    /// Consumes the builder and constructs a [`ScorerSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ScorerSummaryBuilder::created_at)
    /// - [`creator_id`](ScorerSummaryBuilder::creator_id)
    /// - [`is_ootb`](ScorerSummaryBuilder::is_ootb)
    /// - [`kind`](ScorerSummaryBuilder::kind)
    /// - [`name`](ScorerSummaryBuilder::name)
    /// - [`slug`](ScorerSummaryBuilder::slug)
    /// - [`updated_at`](ScorerSummaryBuilder::updated_at)
    pub fn build(self) -> Result<ScorerSummary, BuildError> {
        Ok(ScorerSummary {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            creator_id: self
                .creator_id
                .ok_or_else(|| BuildError::missing_field("creator_id"))?,
            creator_type: self.creator_type,
            description: self.description,
            is_ootb: self
                .is_ootb
                .ok_or_else(|| BuildError::missing_field("is_ootb"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
