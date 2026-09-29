pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateDefinition {
    /// Name of the update
    #[serde(default)]
    pub name: String,
    /// Description of the update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Input JSON schema of the update's model
    #[serde(default)]
    pub input_schema: HashMap<String, serde_json::Value>,
    /// Output JSON schema of the update's model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<HashMap<String, serde_json::Value>>,
}

impl UpdateDefinition {
    pub fn builder() -> UpdateDefinitionBuilder {
        <UpdateDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDefinitionBuilder {
    name: Option<String>,
    description: Option<String>,
    input_schema: Option<HashMap<String, serde_json::Value>>,
    output_schema: Option<HashMap<String, serde_json::Value>>,
}

impl UpdateDefinitionBuilder {
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

    pub fn output_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.output_schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateDefinitionBuilder::name)
    /// - [`input_schema`](UpdateDefinitionBuilder::input_schema)
    pub fn build(self) -> Result<UpdateDefinition, BuildError> {
        Ok(UpdateDefinition {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            input_schema: self
                .input_schema
                .ok_or_else(|| BuildError::missing_field("input_schema"))?,
            output_schema: self.output_schema,
        })
    }
}
