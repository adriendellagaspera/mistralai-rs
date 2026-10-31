pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesText {
    pub r#type: ResponsesTextType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesText {
    pub fn builder() -> ResponsesTextBuilder {
        <ResponsesTextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesTextBuilder {
    r#type: Option<ResponsesTextType>,
}

impl ResponsesTextBuilder {
    pub fn r#type(mut self, value: ResponsesTextType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesText`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ResponsesTextBuilder::r#type)
    pub fn build(self) -> Result<ResponsesText, BuildError> {
        Ok(ResponsesText {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
