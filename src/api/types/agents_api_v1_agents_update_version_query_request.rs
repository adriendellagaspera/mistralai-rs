pub use crate::prelude::*;

/// Query parameters for agents_api_v1_agents_update_version
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsApiV1AgentsUpdateVersionQueryRequest {
    #[serde(default)]
    pub version: i64,
}

impl AgentsApiV1AgentsUpdateVersionQueryRequest {
    pub fn builder() -> AgentsApiV1AgentsUpdateVersionQueryRequestBuilder {
        <AgentsApiV1AgentsUpdateVersionQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsApiV1AgentsUpdateVersionQueryRequestBuilder {
    version: Option<i64>,
}

impl AgentsApiV1AgentsUpdateVersionQueryRequestBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsApiV1AgentsUpdateVersionQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](AgentsApiV1AgentsUpdateVersionQueryRequestBuilder::version)
    pub fn build(self) -> Result<AgentsApiV1AgentsUpdateVersionQueryRequest, BuildError> {
        Ok(AgentsApiV1AgentsUpdateVersionQueryRequest {
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
