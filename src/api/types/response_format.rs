pub use crate::prelude::*;

/// Specify the format that the model must output. By default it will use `{ "type": "text" }`. Setting to `{ "type": "json_object" }` enables JSON mode, which guarantees the message the model generates is in JSON. When using JSON mode you MUST also instruct the model to produce JSON yourself with a system or a user message. Setting to `{ "type": "json_schema" }` enables JSON schema mode, which guarantees the message the model generates is in JSON and follows the schema you provide.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponseFormat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<JsonSchema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ResponseFormats>,
}

impl ResponseFormat {
    pub fn builder() -> ResponseFormatBuilder {
        <ResponseFormatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponseFormatBuilder {
    json_schema: Option<JsonSchema>,
    r#type: Option<ResponseFormats>,
}

impl ResponseFormatBuilder {
    pub fn json_schema(mut self, value: JsonSchema) -> Self {
        self.json_schema = Some(value);
        self
    }

    pub fn r#type(mut self, value: ResponseFormats) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponseFormat`].
    pub fn build(self) -> Result<ResponseFormat, BuildError> {
        Ok(ResponseFormat {
            json_schema: self.json_schema,
            r#type: self.r#type,
        })
    }
}
