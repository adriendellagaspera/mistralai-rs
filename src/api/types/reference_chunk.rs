pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferenceChunk {
    #[serde(default)]
    pub reference_ids: Vec<ReferenceChunkReferenceIdsItem>,
}

impl ReferenceChunk {
    pub fn builder() -> ReferenceChunkBuilder {
        <ReferenceChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferenceChunkBuilder {
    reference_ids: Option<Vec<ReferenceChunkReferenceIdsItem>>,
}

impl ReferenceChunkBuilder {
    pub fn reference_ids(mut self, value: Vec<ReferenceChunkReferenceIdsItem>) -> Self {
        self.reference_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReferenceChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference_ids`](ReferenceChunkBuilder::reference_ids)
    pub fn build(self) -> Result<ReferenceChunk, BuildError> {
        Ok(ReferenceChunk {
            reference_ids: self
                .reference_ids
                .ok_or_else(|| BuildError::missing_field("reference_ids"))?,
        })
    }
}
