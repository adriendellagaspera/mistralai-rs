pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesContent {
    #[serde(default)]
    pub text: String,
    pub r#type: ResponsesContentType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesContent {
    pub fn builder() -> ResponsesContentBuilder {
        <ResponsesContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesContentBuilder {
    text: Option<String>,
    r#type: Option<ResponsesContentType>,
}

impl ResponsesContentBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ResponsesContentType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesContent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](ResponsesContentBuilder::text)
    /// - [`r#type`](ResponsesContentBuilder::r#type)
    pub fn build(self) -> Result<ResponsesContent, BuildError> {
        Ok(ResponsesContent {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
