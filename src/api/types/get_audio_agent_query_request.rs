pub use crate::prelude::*;

/// Query parameters for get_audio_agent
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAudioAgentQueryRequest {
    /// Version to return. Defaults to the latest version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl GetAudioAgentQueryRequest {
    pub fn builder() -> GetAudioAgentQueryRequestBuilder {
        <GetAudioAgentQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAudioAgentQueryRequestBuilder {
    version: Option<i64>,
}

impl GetAudioAgentQueryRequestBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAudioAgentQueryRequest`].
    pub fn build(self) -> Result<GetAudioAgentQueryRequest, BuildError> {
        Ok(GetAudioAgentQueryRequest {
            version: self.version,
        })
    }
}
