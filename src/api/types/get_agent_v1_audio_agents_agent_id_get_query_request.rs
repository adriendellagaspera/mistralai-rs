pub use crate::prelude::*;

/// Query parameters for get_agent_v1_audio_agents__agent_id__get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAgentV1AudioAgentsAgentIdGetQueryRequest {
    /// Version to return. Defaults to the latest version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl GetAgentV1AudioAgentsAgentIdGetQueryRequest {
    pub fn builder() -> GetAgentV1AudioAgentsAgentIdGetQueryRequestBuilder {
        <GetAgentV1AudioAgentsAgentIdGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAgentV1AudioAgentsAgentIdGetQueryRequestBuilder {
    version: Option<i64>,
}

impl GetAgentV1AudioAgentsAgentIdGetQueryRequestBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAgentV1AudioAgentsAgentIdGetQueryRequest`].
    pub fn build(self) -> Result<GetAgentV1AudioAgentsAgentIdGetQueryRequest, BuildError> {
        Ok(GetAgentV1AudioAgentsAgentIdGetQueryRequest {
            version: self.version,
        })
    }
}
