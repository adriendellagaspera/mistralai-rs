pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesFormatTextJsonSchemaConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub schema: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    pub r#type: ResponsesFormatTextJsonSchemaConfigType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesFormatTextJsonSchemaConfig {
    pub fn builder() -> ResponsesFormatTextJsonSchemaConfigBuilder {
        <ResponsesFormatTextJsonSchemaConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesFormatTextJsonSchemaConfigBuilder {
    description: Option<String>,
    name: Option<String>,
    schema: Option<HashMap<String, serde_json::Value>>,
    strict: Option<bool>,
    r#type: Option<ResponsesFormatTextJsonSchemaConfigType>,
}

impl ResponsesFormatTextJsonSchemaConfigBuilder {
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

    pub fn r#type(mut self, value: ResponsesFormatTextJsonSchemaConfigType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesFormatTextJsonSchemaConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ResponsesFormatTextJsonSchemaConfigBuilder::name)
    /// - [`schema`](ResponsesFormatTextJsonSchemaConfigBuilder::schema)
    /// - [`r#type`](ResponsesFormatTextJsonSchemaConfigBuilder::r#type)
    pub fn build(self) -> Result<ResponsesFormatTextJsonSchemaConfig, BuildError> {
        Ok(ResponsesFormatTextJsonSchemaConfig {
            description: self.description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            schema: self
                .schema
                .ok_or_else(|| BuildError::missing_field("schema"))?,
            strict: self.strict,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
