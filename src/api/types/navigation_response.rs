pub use crate::prelude::*;

/// Unranked chunk list in reading order (ascending start_offset).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NavigationResponse {
    #[serde(default)]
    pub chunks: Vec<NavigationChunkResponse>,
}

impl NavigationResponse {
    pub fn builder() -> NavigationResponseBuilder {
        <NavigationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NavigationResponseBuilder {
    chunks: Option<Vec<NavigationChunkResponse>>,
}

impl NavigationResponseBuilder {
    pub fn chunks(mut self, value: Vec<NavigationChunkResponse>) -> Self {
        self.chunks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NavigationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunks`](NavigationResponseBuilder::chunks)
    pub fn build(self) -> Result<NavigationResponse, BuildError> {
        Ok(NavigationResponse {
            chunks: self
                .chunks
                .ok_or_else(|| BuildError::missing_field("chunks"))?,
        })
    }
}
