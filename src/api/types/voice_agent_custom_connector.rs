pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceAgentCustomConnector {
    /// Deprecated: per-request connector credentials will be removed. Authenticate the connector through the Connectors API instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<VoiceAgentCustomConnectorAuthorization>,
    #[serde(default)]
    pub connector_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filler: Option<VoiceAgentCustomConnectorFiller>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_configuration: Option<ToolConfiguration>,
}

impl VoiceAgentCustomConnector {
    pub fn builder() -> VoiceAgentCustomConnectorBuilder {
        <VoiceAgentCustomConnectorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceAgentCustomConnectorBuilder {
    authorization: Option<VoiceAgentCustomConnectorAuthorization>,
    connector_id: Option<String>,
    filler: Option<VoiceAgentCustomConnectorFiller>,
    tool_configuration: Option<ToolConfiguration>,
}

impl VoiceAgentCustomConnectorBuilder {
    pub fn authorization(mut self, value: VoiceAgentCustomConnectorAuthorization) -> Self {
        self.authorization = Some(value);
        self
    }

    pub fn connector_id(mut self, value: impl Into<String>) -> Self {
        self.connector_id = Some(value.into());
        self
    }

    pub fn filler(mut self, value: VoiceAgentCustomConnectorFiller) -> Self {
        self.filler = Some(value);
        self
    }

    pub fn tool_configuration(mut self, value: ToolConfiguration) -> Self {
        self.tool_configuration = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceAgentCustomConnector`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connector_id`](VoiceAgentCustomConnectorBuilder::connector_id)
    pub fn build(self) -> Result<VoiceAgentCustomConnector, BuildError> {
        Ok(VoiceAgentCustomConnector {
            authorization: self.authorization,
            connector_id: self
                .connector_id
                .ok_or_else(|| BuildError::missing_field("connector_id"))?,
            filler: self.filler,
            tool_configuration: self.tool_configuration,
        })
    }
}
