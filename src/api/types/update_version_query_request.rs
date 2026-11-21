pub use crate::prelude::*;

/// Query parameters for update_version
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateVersionQueryRequest {
    #[serde(default)]
    pub version: i64,
}

impl UpdateVersionQueryRequest {
    pub fn builder() -> UpdateVersionQueryRequestBuilder {
        <UpdateVersionQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateVersionQueryRequestBuilder {
    version: Option<i64>,
}

impl UpdateVersionQueryRequestBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateVersionQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](UpdateVersionQueryRequestBuilder::version)
    pub fn build(self) -> Result<UpdateVersionQueryRequest, BuildError> {
        Ok(UpdateVersionQueryRequest {
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
