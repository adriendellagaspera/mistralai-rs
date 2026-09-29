pub use crate::prelude::*;

/// Query parameters for agents_api_v1_agents_delete_alias
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsApiV1AgentsDeleteAliasQueryRequest {
    #[serde(default)]
    pub alias: String,
}

impl AgentsApiV1AgentsDeleteAliasQueryRequest {
    pub fn builder() -> AgentsApiV1AgentsDeleteAliasQueryRequestBuilder {
        <AgentsApiV1AgentsDeleteAliasQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsApiV1AgentsDeleteAliasQueryRequestBuilder {
    alias: Option<String>,
}

impl AgentsApiV1AgentsDeleteAliasQueryRequestBuilder {
    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsApiV1AgentsDeleteAliasQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alias`](AgentsApiV1AgentsDeleteAliasQueryRequestBuilder::alias)
    pub fn build(self) -> Result<AgentsApiV1AgentsDeleteAliasQueryRequest, BuildError> {
        Ok(AgentsApiV1AgentsDeleteAliasQueryRequest {
            alias: self
                .alias
                .ok_or_else(|| BuildError::missing_field("alias"))?,
        })
    }
}
