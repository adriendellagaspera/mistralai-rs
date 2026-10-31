pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManagedIndexConfig {
    pub embedding: ManagedIndexConfigEmbedding,
}

impl ManagedIndexConfig {
    pub fn builder() -> ManagedIndexConfigBuilder {
        <ManagedIndexConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManagedIndexConfigBuilder {
    embedding: Option<ManagedIndexConfigEmbedding>,
}

impl ManagedIndexConfigBuilder {
    pub fn embedding(mut self, value: ManagedIndexConfigEmbedding) -> Self {
        self.embedding = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ManagedIndexConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`embedding`](ManagedIndexConfigBuilder::embedding)
    pub fn build(self) -> Result<ManagedIndexConfig, BuildError> {
        Ok(ManagedIndexConfig {
            embedding: self
                .embedding
                .ok_or_else(|| BuildError::missing_field("embedding"))?,
        })
    }
}
