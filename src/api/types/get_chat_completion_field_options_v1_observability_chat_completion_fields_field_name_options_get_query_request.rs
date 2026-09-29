pub use crate::prelude::*;

/// Query parameters for get_chat_completion_field_options_v1_observability_chat_completion_fields__field_name__options_get
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetQueryRequest {
    /// The operator to use for filtering options
    pub operator: GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetFieldsRequestOperator,
}

impl
    GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetQueryRequest
{
    pub fn builder() -> GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetQueryRequestBuilder{
        <GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetQueryRequestBuilder {
    operator: Option<GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetFieldsRequestOperator>,
}

impl GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetQueryRequestBuilder {
    pub fn operator(mut self, value: GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetFieldsRequestOperator) -> Self {
        self.operator = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`operator`](GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetQueryRequestBuilder::operator)
    pub fn build(self) -> Result<GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetQueryRequest, BuildError> {
        Ok(GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetQueryRequest {
            operator: self.operator.ok_or_else(|| BuildError::missing_field("operator"))?,
        })
    }
}
