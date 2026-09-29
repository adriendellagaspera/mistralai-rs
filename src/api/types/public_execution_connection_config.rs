pub use crate::prelude::*;

/// Connection config exposed in the public, unauthenticated /connectors/mistral response.
///
/// Unlike ConnectionConfig, this has no `headers` field and forbids extra fields, so
/// connector credentials can never be serialized into this cacheable response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicExecutionConnectionConfig {
    pub r#type: ConnectionConfigType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_configuration: Option<ToolExecutionConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_internally: Option<bool>,
}

impl PublicExecutionConnectionConfig {
    pub fn builder() -> PublicExecutionConnectionConfigBuilder {
        <PublicExecutionConnectionConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicExecutionConnectionConfigBuilder {
    r#type: Option<ConnectionConfigType>,
    server: Option<String>,
    name: Option<String>,
    id: Option<String>,
    tool_configuration: Option<ToolExecutionConfiguration>,
    hosted_internally: Option<bool>,
}

impl PublicExecutionConnectionConfigBuilder {
    pub fn r#type(mut self, value: ConnectionConfigType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn server(mut self, value: impl Into<String>) -> Self {
        self.server = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn tool_configuration(mut self, value: ToolExecutionConfiguration) -> Self {
        self.tool_configuration = Some(value);
        self
    }

    pub fn hosted_internally(mut self, value: bool) -> Self {
        self.hosted_internally = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicExecutionConnectionConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PublicExecutionConnectionConfigBuilder::r#type)
    pub fn build(self) -> Result<PublicExecutionConnectionConfig, BuildError> {
        Ok(PublicExecutionConnectionConfig {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            server: self.server,
            name: self.name,
            id: self.id,
            tool_configuration: self.tool_configuration,
            hosted_internally: self.hosted_internally,
        })
    }
}
