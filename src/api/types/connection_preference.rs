pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionPreference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_type: Option<ConsumerType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default)]
    pub name: String,
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
    is_default: Option<bool>,
    name: Option<String>,
    tool_configuration: Option<ToolExecutionConfiguration>,
}

impl ConnectionPreferenceBuilder {
    pub fn consumer_type(mut self, value: ConsumerType) -> Self {
        self.consumer_type = Some(value);
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
    /// - [`name`](ConnectionPreferenceBuilder::name)
    /// - [`tool_configuration`](ConnectionPreferenceBuilder::tool_configuration)
    pub fn build(self) -> Result<ConnectionPreference, BuildError> {
        Ok(ConnectionPreference {
            consumer_type: self.consumer_type,
            is_default: self.is_default,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tool_configuration: self
                .tool_configuration
                .ok_or_else(|| BuildError::missing_field("tool_configuration"))?,
        })
    }
}
