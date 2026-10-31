pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateManagedIndexRequest {
    pub config: ManagedIndexConfig,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ManagedIndexFields>,
}

impl CreateManagedIndexRequest {
    pub fn builder() -> CreateManagedIndexRequestBuilder {
        <CreateManagedIndexRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateManagedIndexRequestBuilder {
    config: Option<ManagedIndexConfig>,
    name: Option<String>,
    schema: Option<ManagedIndexFields>,
}

impl CreateManagedIndexRequestBuilder {
    pub fn config(mut self, value: ManagedIndexConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn schema(mut self, value: ManagedIndexFields) -> Self {
        self.schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateManagedIndexRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](CreateManagedIndexRequestBuilder::config)
    /// - [`name`](CreateManagedIndexRequestBuilder::name)
    pub fn build(self) -> Result<CreateManagedIndexRequest, BuildError> {
        Ok(CreateManagedIndexRequest {
            config: self
                .config
                .ok_or_else(|| BuildError::missing_field("config"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            schema: self.schema,
        })
    }
}
