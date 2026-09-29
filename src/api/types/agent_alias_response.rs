pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentAliasResponse {
    #[serde(default)]
    pub alias: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub version: i64,
}

impl AgentAliasResponse {
    pub fn builder() -> AgentAliasResponseBuilder {
        <AgentAliasResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentAliasResponseBuilder {
    alias: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    version: Option<i64>,
}

impl AgentAliasResponseBuilder {
    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
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

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentAliasResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alias`](AgentAliasResponseBuilder::alias)
    /// - [`created_at`](AgentAliasResponseBuilder::created_at)
    /// - [`updated_at`](AgentAliasResponseBuilder::updated_at)
    /// - [`version`](AgentAliasResponseBuilder::version)
    pub fn build(self) -> Result<AgentAliasResponse, BuildError> {
        Ok(AgentAliasResponse {
            alias: self
                .alias
                .ok_or_else(|| BuildError::missing_field("alias"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
