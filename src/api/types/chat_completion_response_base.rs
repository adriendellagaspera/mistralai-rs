pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChatCompletionResponseBase {
    #[serde(flatten)]
    pub response_base_fields: ResponseBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
}

impl ChatCompletionResponseBase {
    pub fn builder() -> ChatCompletionResponseBaseBuilder {
        <ChatCompletionResponseBaseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionResponseBaseBuilder {
    response_base_fields: Option<ResponseBase>,
    created: Option<i64>,
}

impl ChatCompletionResponseBaseBuilder {
    pub fn response_base_fields(mut self, value: ResponseBase) -> Self {
        self.response_base_fields = Some(value);
        self
    }

    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionResponseBase`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_base_fields`](ChatCompletionResponseBaseBuilder::response_base_fields)
    pub fn build(self) -> Result<ChatCompletionResponseBase, BuildError> {
        Ok(ChatCompletionResponseBase {
            response_base_fields: self
                .response_base_fields
                .ok_or_else(|| BuildError::missing_field("response_base_fields"))?,
            created: self.created,
        })
    }
}
