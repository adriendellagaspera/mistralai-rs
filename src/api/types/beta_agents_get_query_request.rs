pub use crate::prelude::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaAgentsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<GetAgentsRequestAgentVersion>,
}

impl BetaAgentsGetQueryRequest {
    pub fn builder() -> BetaAgentsGetQueryRequestBuilder {
        <BetaAgentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaAgentsGetQueryRequestBuilder {
    agent_version: Option<GetAgentsRequestAgentVersion>,
}

impl BetaAgentsGetQueryRequestBuilder {
    pub fn agent_version(mut self, value: GetAgentsRequestAgentVersion) -> Self {
        self.agent_version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BetaAgentsGetQueryRequest`].
    pub fn build(self) -> Result<BetaAgentsGetQueryRequest, BuildError> {
        Ok(BetaAgentsGetQueryRequest {
            agent_version: self.agent_version,
        })
    }
}
