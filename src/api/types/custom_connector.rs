pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomConnector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<CustomConnectorAuthorization>,
    #[serde(default)]
    pub connector_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_configuration: Option<ToolConfiguration>,
}

impl CustomConnector {
    pub fn builder() -> CustomConnectorBuilder {
        <CustomConnectorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomConnectorBuilder {
    authorization: Option<CustomConnectorAuthorization>,
    connector_id: Option<String>,
    tool_configuration: Option<ToolConfiguration>,
}

impl CustomConnectorBuilder {
    pub fn authorization(mut self, value: CustomConnectorAuthorization) -> Self {
        self.authorization = Some(value);
        self
    }

    pub fn connector_id(mut self, value: impl Into<String>) -> Self {
        self.connector_id = Some(value.into());
        self
    }

    pub fn tool_configuration(mut self, value: ToolConfiguration) -> Self {
        self.tool_configuration = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomConnector`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connector_id`](CustomConnectorBuilder::connector_id)
    pub fn build(self) -> Result<CustomConnector, BuildError> {
        Ok(CustomConnector {
            authorization: self.authorization,
            connector_id: self
                .connector_id
                .ok_or_else(|| BuildError::missing_field("connector_id"))?,
            tool_configuration: self.tool_configuration,
        })
    }
}
