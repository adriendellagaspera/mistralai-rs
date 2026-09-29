pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicExecutionConnector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_config: Option<PublicExecutionConnectionConfig>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl PublicExecutionConnector {
    pub fn builder() -> PublicExecutionConnectorBuilder {
        <PublicExecutionConnectorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicExecutionConnectorBuilder {
    connection_config: Option<PublicExecutionConnectionConfig>,
    id: Option<String>,
    name: Option<String>,
}

impl PublicExecutionConnectorBuilder {
    pub fn connection_config(mut self, value: PublicExecutionConnectionConfig) -> Self {
        self.connection_config = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PublicExecutionConnector`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PublicExecutionConnectorBuilder::id)
    /// - [`name`](PublicExecutionConnectorBuilder::name)
    pub fn build(self) -> Result<PublicExecutionConnector, BuildError> {
        Ok(PublicExecutionConnector {
            connection_config: self.connection_config,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
