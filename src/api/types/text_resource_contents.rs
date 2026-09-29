pub use crate::prelude::*;

/// Text contents of a resource.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TextResourceContents {
    #[serde(default)]
    pub uri: String,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub text: String,
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
    uri: Option<String>,
    mime_type: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    text: Option<String>,
}

impl TextResourceContentsBuilder {
    pub fn uri(mut self, value: impl Into<String>) -> Self {
        self.uri = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TextResourceContents`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uri`](TextResourceContentsBuilder::uri)
    /// - [`text`](TextResourceContentsBuilder::text)
    pub fn build(self) -> Result<TextResourceContents, BuildError> {
        Ok(TextResourceContents {
            uri: self.uri.ok_or_else(|| BuildError::missing_field("uri"))?,
            mime_type: self.mime_type,
            meta: self.meta,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            extra: Default::default(),
        })
    }
}
