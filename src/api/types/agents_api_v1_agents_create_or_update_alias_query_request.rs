pub use crate::prelude::*;

/// Query parameters for agents_api_v1_agents_create_or_update_alias
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsApiV1AgentsCreateOrUpdateAliasQueryRequest {
    #[serde(default)]
    pub alias: String,
    #[serde(default)]
    pub version: i64,
}

impl AgentsApiV1AgentsCreateOrUpdateAliasQueryRequest {
    pub fn builder() -> AgentsApiV1AgentsCreateOrUpdateAliasQueryRequestBuilder {
        <AgentsApiV1AgentsCreateOrUpdateAliasQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsApiV1AgentsCreateOrUpdateAliasQueryRequestBuilder {
    alias: Option<String>,
    version: Option<i64>,
}

impl AgentsApiV1AgentsCreateOrUpdateAliasQueryRequestBuilder {
    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsApiV1AgentsCreateOrUpdateAliasQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alias`](AgentsApiV1AgentsCreateOrUpdateAliasQueryRequestBuilder::alias)
    /// - [`version`](AgentsApiV1AgentsCreateOrUpdateAliasQueryRequestBuilder::version)
    pub fn build(self) -> Result<AgentsApiV1AgentsCreateOrUpdateAliasQueryRequest, BuildError> {
        Ok(AgentsApiV1AgentsCreateOrUpdateAliasQueryRequest {
            alias: self
                .alias
                .ok_or_else(|| BuildError::missing_field("alias"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
