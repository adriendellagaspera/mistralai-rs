pub use crate::prelude::*;

/// Query parameters for create_version_alias
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateVersionAliasQueryRequest {
    #[serde(default)]
    pub alias: String,
    #[serde(default)]
    pub version: i64,
}

impl CreateVersionAliasQueryRequest {
    pub fn builder() -> CreateVersionAliasQueryRequestBuilder {
        <CreateVersionAliasQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateVersionAliasQueryRequestBuilder {
    alias: Option<String>,
    version: Option<i64>,
}

impl CreateVersionAliasQueryRequestBuilder {
    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateVersionAliasQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alias`](CreateVersionAliasQueryRequestBuilder::alias)
    /// - [`version`](CreateVersionAliasQueryRequestBuilder::version)
    pub fn build(self) -> Result<CreateVersionAliasQueryRequest, BuildError> {
        Ok(CreateVersionAliasQueryRequest {
            alias: self
                .alias
                .ok_or_else(|| BuildError::missing_field("alias"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
