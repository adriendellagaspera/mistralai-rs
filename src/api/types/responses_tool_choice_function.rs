pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesToolChoiceFunction {
    #[serde(default)]
    pub name: String,
    pub r#type: ResponsesToolChoiceFunctionType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesToolChoiceFunction {
    pub fn builder() -> ResponsesToolChoiceFunctionBuilder {
        <ResponsesToolChoiceFunctionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesToolChoiceFunctionBuilder {
    name: Option<String>,
    r#type: Option<ResponsesToolChoiceFunctionType>,
}

impl ResponsesToolChoiceFunctionBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ResponsesToolChoiceFunctionType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesToolChoiceFunction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ResponsesToolChoiceFunctionBuilder::name)
    /// - [`r#type`](ResponsesToolChoiceFunctionBuilder::r#type)
    pub fn build(self) -> Result<ResponsesToolChoiceFunction, BuildError> {
        Ok(ResponsesToolChoiceFunction {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
