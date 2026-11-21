pub use crate::prelude::*;

/// Query parameters for delete_version_alias
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteVersionAliasQueryRequest {
    #[serde(default)]
    pub alias: String,
}

impl DeleteVersionAliasQueryRequest {
    pub fn builder() -> DeleteVersionAliasQueryRequestBuilder {
        <DeleteVersionAliasQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteVersionAliasQueryRequestBuilder {
    alias: Option<String>,
}

impl DeleteVersionAliasQueryRequestBuilder {
    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteVersionAliasQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alias`](DeleteVersionAliasQueryRequestBuilder::alias)
    pub fn build(self) -> Result<DeleteVersionAliasQueryRequest, BuildError> {
        Ok(DeleteVersionAliasQueryRequest {
            alias: self
                .alias
                .ok_or_else(|| BuildError::missing_field("alias"))?,
        })
    }
}
