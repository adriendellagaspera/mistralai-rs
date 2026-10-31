pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpanDatasetSourceReference {
    /// Literal attribute dictionary key; dots do not denote a nested path.
    #[serde(default)]
    pub key: String,
    pub namespace: SpanDatasetSourceNamespace,
}

impl SpanDatasetSourceReference {
    pub fn builder() -> SpanDatasetSourceReferenceBuilder {
        <SpanDatasetSourceReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpanDatasetSourceReferenceBuilder {
    key: Option<String>,
    namespace: Option<SpanDatasetSourceNamespace>,
}

impl SpanDatasetSourceReferenceBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn namespace(mut self, value: SpanDatasetSourceNamespace) -> Self {
        self.namespace = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpanDatasetSourceReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](SpanDatasetSourceReferenceBuilder::key)
    /// - [`namespace`](SpanDatasetSourceReferenceBuilder::namespace)
    pub fn build(self) -> Result<SpanDatasetSourceReference, BuildError> {
        Ok(SpanDatasetSourceReference {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            namespace: self
                .namespace
                .ok_or_else(|| BuildError::missing_field("namespace"))?,
        })
    }
}
