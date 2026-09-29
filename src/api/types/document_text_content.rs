pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentTextContent {
    #[serde(default)]
    pub text: String,
}

impl DocumentTextContent {
    pub fn builder() -> DocumentTextContentBuilder {
        <DocumentTextContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentTextContentBuilder {
    text: Option<String>,
}

impl DocumentTextContentBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentTextContent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](DocumentTextContentBuilder::text)
    pub fn build(self) -> Result<DocumentTextContent, BuildError> {
        Ok(DocumentTextContent {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
