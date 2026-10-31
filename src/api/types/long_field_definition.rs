pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LongFieldDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
}

impl LongFieldDefinition {
    pub fn builder() -> LongFieldDefinitionBuilder {
        <LongFieldDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LongFieldDefinitionBuilder {
    default: Option<serde_json::Value>,
    indexed: Option<bool>,
    required: Option<bool>,
    system: Option<bool>,
}

impl LongFieldDefinitionBuilder {
    pub fn default(mut self, value: serde_json::Value) -> Self {
        self.default = Some(value);
        self
    }

    pub fn indexed(mut self, value: bool) -> Self {
        self.indexed = Some(value);
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn system(mut self, value: bool) -> Self {
        self.system = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LongFieldDefinition`].
    pub fn build(self) -> Result<LongFieldDefinition, BuildError> {
        Ok(LongFieldDefinition {
            default: self.default,
            indexed: self.indexed,
            required: self.required,
            system: self.system,
        })
    }
}
