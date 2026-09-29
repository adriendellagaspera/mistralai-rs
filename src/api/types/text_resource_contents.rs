pub use crate::prelude::*;

/// Text contents of a resource.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TextResourceContents {
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub uri: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl TextResourceContents {
    pub fn builder() -> TextResourceContentsBuilder {
        <TextResourceContentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextResourceContentsBuilder {
    meta: Option<HashMap<String, serde_json::Value>>,
    mime_type: Option<String>,
    text: Option<String>,
    uri: Option<String>,
}

impl TextResourceContentsBuilder {
    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn uri(mut self, value: impl Into<String>) -> Self {
        self.uri = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TextResourceContents`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TextResourceContentsBuilder::text)
    /// - [`uri`](TextResourceContentsBuilder::uri)
    pub fn build(self) -> Result<TextResourceContents, BuildError> {
        Ok(TextResourceContents {
            meta: self.meta,
            mime_type: self.mime_type,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            uri: self.uri.ok_or_else(|| BuildError::missing_field("uri"))?,
            extra: Default::default(),
        })
    }
}
