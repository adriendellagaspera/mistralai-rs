pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterSearchIndexResponseIndex {
    #[serde(default)]
    pub id: String,
}

impl RegisterSearchIndexResponseIndex {
    pub fn builder() -> RegisterSearchIndexResponseIndexBuilder {
        <RegisterSearchIndexResponseIndexBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterSearchIndexResponseIndexBuilder {
    id: Option<String>,
}

impl RegisterSearchIndexResponseIndexBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RegisterSearchIndexResponseIndex`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RegisterSearchIndexResponseIndexBuilder::id)
    pub fn build(self) -> Result<RegisterSearchIndexResponseIndex, BuildError> {
        Ok(RegisterSearchIndexResponseIndex {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
