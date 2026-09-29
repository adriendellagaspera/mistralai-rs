pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectorCallToolRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip)]
    pub credentials_name: Option<String>,
}

impl ConnectorCallToolRequest {
    pub fn builder() -> ConnectorCallToolRequestBuilder {
        <ConnectorCallToolRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorCallToolRequestBuilder {
    arguments: Option<HashMap<String, serde_json::Value>>,
    credentials_name: Option<String>,
}

impl ConnectorCallToolRequestBuilder {
    pub fn arguments(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn credentials_name(mut self, value: impl Into<String>) -> Self {
        self.credentials_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectorCallToolRequest`].
    pub fn build(self) -> Result<ConnectorCallToolRequest, BuildError> {
        Ok(ConnectorCallToolRequest {
            arguments: self.arguments,
            credentials_name: self.credentials_name,
        })
    }
}
