pub use crate::prelude::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioAgentsGetQueryRequest {
    /// Version to return. Defaults to the latest version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl AudioAgentsGetQueryRequest {
    pub fn builder() -> AudioAgentsGetQueryRequestBuilder {
        <AudioAgentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioAgentsGetQueryRequestBuilder {
    version: Option<i64>,
}

impl AudioAgentsGetQueryRequestBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioAgentsGetQueryRequest`].
    pub fn build(self) -> Result<AudioAgentsGetQueryRequest, BuildError> {
        Ok(AudioAgentsGetQueryRequest {
            version: self.version,
        })
    }
}
