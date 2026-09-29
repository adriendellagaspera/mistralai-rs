pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublicExecutionConnector {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_config: Option<PublicExecutionConnectionConfig>,
}

impl PublicExecutionConnector {
    pub fn builder() -> PublicExecutionConnectorBuilder {
        <PublicExecutionConnectorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicExecutionConnectorBuilder {
    id: Option<String>,
    name: Option<String>,
    connection_config: Option<PublicExecutionConnectionConfig>,
}

impl PublicExecutionConnectorBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn connection_config(mut self, value: PublicExecutionConnectionConfig) -> Self {
        self.connection_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicExecutionConnector`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PublicExecutionConnectorBuilder::id)
    /// - [`name`](PublicExecutionConnectorBuilder::name)
    pub fn build(self) -> Result<PublicExecutionConnector, BuildError> {
        Ok(PublicExecutionConnector {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            connection_config: self.connection_config,
        })
    }
}
