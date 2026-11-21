pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionPreference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_type: Option<ConsumerType>,
    /// Whether raw HTTP calls through this connector are allowed at this level. Only applies to HTTP connectors: null when the level has no opinion, always the case for other connectors. False at any level switches them off for this level and the ones below.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_proxy_enabled: Option<bool>,
    /// Deprecated: always `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Deprecated: always `default`, a connection holds a single preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub tool_configuration: ToolExecutionConfiguration,
}

impl ConnectionPreference {
    pub fn builder() -> ConnectionPreferenceBuilder {
        <ConnectionPreferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionPreferenceBuilder {
    consumer_type: Option<ConsumerType>,
    http_proxy_enabled: Option<bool>,
    is_default: Option<bool>,
    name: Option<String>,
    tool_configuration: Option<ToolExecutionConfiguration>,
}

impl ConnectionPreferenceBuilder {
    pub fn consumer_type(mut self, value: ConsumerType) -> Self {
        self.consumer_type = Some(value);
        self
    }

    pub fn http_proxy_enabled(mut self, value: bool) -> Self {
        self.http_proxy_enabled = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tool_configuration(mut self, value: ToolExecutionConfiguration) -> Self {
        self.tool_configuration = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectionPreference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_configuration`](ConnectionPreferenceBuilder::tool_configuration)
    pub fn build(self) -> Result<ConnectionPreference, BuildError> {
        Ok(ConnectionPreference {
            consumer_type: self.consumer_type,
            http_proxy_enabled: self.http_proxy_enabled,
            is_default: self.is_default,
            name: self.name,
            tool_configuration: self
                .tool_configuration
                .ok_or_else(|| BuildError::missing_field("tool_configuration"))?,
        })
    }
}
