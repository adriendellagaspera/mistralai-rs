pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionPreference {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub tool_configuration: ToolExecutionConfiguration,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_type: Option<ConsumerType>,
}

impl ConnectionPreference {
    pub fn builder() -> ConnectionPreferenceBuilder {
        <ConnectionPreferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionPreferenceBuilder {
    name: Option<String>,
    tool_configuration: Option<ToolExecutionConfiguration>,
    is_default: Option<bool>,
    consumer_type: Option<ConsumerType>,
}

impl ConnectionPreferenceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tool_configuration(mut self, value: ToolExecutionConfiguration) -> Self {
        self.tool_configuration = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn consumer_type(mut self, value: ConsumerType) -> Self {
        self.consumer_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectionPreference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConnectionPreferenceBuilder::name)
    /// - [`tool_configuration`](ConnectionPreferenceBuilder::tool_configuration)
    pub fn build(self) -> Result<ConnectionPreference, BuildError> {
        Ok(ConnectionPreference {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tool_configuration: self
                .tool_configuration
                .ok_or_else(|| BuildError::missing_field("tool_configuration"))?,
            is_default: self.is_default,
            consumer_type: self.consumer_type,
        })
    }
}
