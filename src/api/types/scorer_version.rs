pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScorerVersion {
    pub config: LlmJudgeConfig,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub creator_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_type: Option<PrincipalType>,
    #[serde(default)]
    pub variables: Vec<String>,
    #[serde(default)]
    pub version: i64,
}

impl ScorerVersion {
    pub fn builder() -> ScorerVersionBuilder {
        <ScorerVersionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScorerVersionBuilder {
    config: Option<LlmJudgeConfig>,
    created_at: Option<DateTime<FixedOffset>>,
    creator_id: Option<String>,
    creator_type: Option<PrincipalType>,
    variables: Option<Vec<String>>,
    version: Option<i64>,
}

impl ScorerVersionBuilder {
    pub fn config(mut self, value: LlmJudgeConfig) -> Self {
        self.config = Some(value);
        self
    }

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

    pub fn variables(mut self, value: Vec<String>) -> Self {
        self.variables = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScorerVersion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](ScorerVersionBuilder::config)
    /// - [`created_at`](ScorerVersionBuilder::created_at)
    /// - [`creator_id`](ScorerVersionBuilder::creator_id)
    /// - [`variables`](ScorerVersionBuilder::variables)
    /// - [`version`](ScorerVersionBuilder::version)
    pub fn build(self) -> Result<ScorerVersion, BuildError> {
        Ok(ScorerVersion {
            config: self
                .config
                .ok_or_else(|| BuildError::missing_field("config"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            creator_id: self
                .creator_id
                .ok_or_else(|| BuildError::missing_field("creator_id"))?,
            creator_type: self.creator_type,
            variables: self
                .variables
                .ok_or_else(|| BuildError::missing_field("variables"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
