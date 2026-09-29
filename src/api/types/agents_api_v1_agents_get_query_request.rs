pub use crate::prelude::*;

/// Query parameters for agents_api_v1_agents_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsApiV1AgentsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<AgentsApiV1AgentsGetAgentsRequestAgentVersion>,
}

impl AgentsApiV1AgentsGetQueryRequest {
    pub fn builder() -> AgentsApiV1AgentsGetQueryRequestBuilder {
        <AgentsApiV1AgentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsApiV1AgentsGetQueryRequestBuilder {
    agent_version: Option<AgentsApiV1AgentsGetAgentsRequestAgentVersion>,
}

impl AgentsApiV1AgentsGetQueryRequestBuilder {
    pub fn agent_version(mut self, value: AgentsApiV1AgentsGetAgentsRequestAgentVersion) -> Self {
        self.agent_version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsApiV1AgentsGetQueryRequest`].
    pub fn build(self) -> Result<AgentsApiV1AgentsGetQueryRequest, BuildError> {
        Ok(AgentsApiV1AgentsGetQueryRequest {
            agent_version: self.agent_version,
        })
    }
}
