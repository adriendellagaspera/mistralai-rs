pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryDefinition {
    /// Description of the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Input JSON schema of the query's model
    #[serde(default)]
    pub input_schema: HashMap<String, serde_json::Value>,
    /// Name of the query
    #[serde(default)]
    pub name: String,
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
    description: Option<String>,
    input_schema: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
    output_schema: Option<HashMap<String, serde_json::Value>>,
}

impl QueryDefinitionBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn input_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input_schema = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn output_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.output_schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input_schema`](QueryDefinitionBuilder::input_schema)
    /// - [`name`](QueryDefinitionBuilder::name)
    pub fn build(self) -> Result<QueryDefinition, BuildError> {
        Ok(QueryDefinition {
            description: self.description,
            input_schema: self
                .input_schema
                .ok_or_else(|| BuildError::missing_field("input_schema"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            output_schema: self.output_schema,
        })
    }
}
