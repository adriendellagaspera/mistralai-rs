pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FetchChatCompletionFieldOptionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Option<FetchChatCompletionFieldOptionsResponseOptionsItem>>>,
}

impl FetchChatCompletionFieldOptionsResponse {
    pub fn builder() -> FetchChatCompletionFieldOptionsResponseBuilder {
        <FetchChatCompletionFieldOptionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FetchChatCompletionFieldOptionsResponseBuilder {
    options: Option<Vec<Option<FetchChatCompletionFieldOptionsResponseOptionsItem>>>,
}

impl FetchChatCompletionFieldOptionsResponseBuilder {
    pub fn options(
        mut self,
        value: Vec<Option<FetchChatCompletionFieldOptionsResponseOptionsItem>>,
    ) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FetchChatCompletionFieldOptionsResponse`].
    pub fn build(self) -> Result<FetchChatCompletionFieldOptionsResponse, BuildError> {
        Ok(FetchChatCompletionFieldOptionsResponse {
            options: self.options,
        })
    }
}
