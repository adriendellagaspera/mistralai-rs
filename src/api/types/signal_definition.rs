pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SignalDefinition {
    /// Name of the signal
    #[serde(default)]
    pub name: String,
    /// Description of the signal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Input JSON schema of the signal's model
    #[serde(default)]
    pub input_schema: HashMap<String, serde_json::Value>,
}

impl SignalDefinition {
    pub fn builder() -> SignalDefinitionBuilder {
        <SignalDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignalDefinitionBuilder {
    name: Option<String>,
    description: Option<String>,
    input_schema: Option<HashMap<String, serde_json::Value>>,
}

impl SignalDefinitionBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn input_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input_schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignalDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](SignalDefinitionBuilder::name)
    /// - [`input_schema`](SignalDefinitionBuilder::input_schema)
    pub fn build(self) -> Result<SignalDefinition, BuildError> {
        Ok(SignalDefinition {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            input_schema: self
                .input_schema
                .ok_or_else(|| BuildError::missing_field("input_schema"))?,
        })
    }
}
