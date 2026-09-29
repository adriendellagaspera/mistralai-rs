pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryDefinition {
    /// Name of the query
    #[serde(default)]
    pub name: String,
    /// Description of the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Input JSON schema of the query's model
    #[serde(default)]
    pub input_schema: HashMap<String, serde_json::Value>,
    /// Output JSON schema of the query's model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<HashMap<String, serde_json::Value>>,
}

impl QueryDefinition {
    pub fn builder() -> QueryDefinitionBuilder {
        <QueryDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryDefinitionBuilder {
    name: Option<String>,
    description: Option<String>,
    input_schema: Option<HashMap<String, serde_json::Value>>,
    output_schema: Option<HashMap<String, serde_json::Value>>,
}

impl QueryDefinitionBuilder {
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

    /// Consumes the builder and constructs a [`QueryDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryDefinitionBuilder::name)
    /// - [`input_schema`](QueryDefinitionBuilder::input_schema)
    pub fn build(self) -> Result<QueryDefinition, BuildError> {
        Ok(QueryDefinition {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            input_schema: self
                .input_schema
                .ok_or_else(|| BuildError::missing_field("input_schema"))?,
            output_schema: self.output_schema,
        })
    }
}
