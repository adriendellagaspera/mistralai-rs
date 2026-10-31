pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesFormatText {
    pub r#type: ResponsesFormatTextType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesFormatText {
    pub fn builder() -> ResponsesFormatTextBuilder {
        <ResponsesFormatTextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesFormatTextBuilder {
    r#type: Option<ResponsesFormatTextType>,
}

impl ResponsesFormatTextBuilder {
    pub fn r#type(mut self, value: ResponsesFormatTextType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesFormatText`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ResponsesFormatTextBuilder::r#type)
    pub fn build(self) -> Result<ResponsesFormatText, BuildError> {
        Ok(ResponsesFormatText {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
