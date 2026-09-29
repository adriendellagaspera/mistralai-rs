pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct JsonSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub schema: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

impl JsonSchema {
    pub fn builder() -> JsonSchemaBuilder {
        <JsonSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JsonSchemaBuilder {
    description: Option<String>,
    name: Option<String>,
    schema: Option<HashMap<String, serde_json::Value>>,
    strict: Option<bool>,
}

impl JsonSchemaBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.schema = Some(value);
        self
    }

    pub fn strict(mut self, value: bool) -> Self {
        self.strict = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JsonSchema`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](JsonSchemaBuilder::name)
    /// - [`schema`](JsonSchemaBuilder::schema)
    pub fn build(self) -> Result<JsonSchema, BuildError> {
        Ok(JsonSchema {
            description: self.description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            schema: self
                .schema
                .ok_or_else(|| BuildError::missing_field("schema"))?,
            strict: self.strict,
        })
    }
}
