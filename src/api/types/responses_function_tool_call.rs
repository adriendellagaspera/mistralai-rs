pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesFunctionToolCall {
    #[serde(default)]
    pub arguments: String,
    #[serde(default)]
    pub call_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller: Option<ResponsesFunctionToolCallCaller>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponsesFunctionToolCallStatus>,
    pub r#type: ResponsesFunctionToolCallType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesFunctionToolCall {
    pub fn builder() -> ResponsesFunctionToolCallBuilder {
        <ResponsesFunctionToolCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesFunctionToolCallBuilder {
    arguments: Option<String>,
    call_id: Option<String>,
    caller: Option<ResponsesFunctionToolCallCaller>,
    id: Option<String>,
    name: Option<String>,
    namespace: Option<String>,
    status: Option<ResponsesFunctionToolCallStatus>,
    r#type: Option<ResponsesFunctionToolCallType>,
}

impl ResponsesFunctionToolCallBuilder {
    pub fn arguments(mut self, value: impl Into<String>) -> Self {
        self.arguments = Some(value.into());
        self
    }

    pub fn call_id(mut self, value: impl Into<String>) -> Self {
        self.call_id = Some(value.into());
        self
    }

    pub fn caller(mut self, value: ResponsesFunctionToolCallCaller) -> Self {
        self.caller = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn namespace(mut self, value: impl Into<String>) -> Self {
        self.namespace = Some(value.into());
        self
    }

    pub fn status(mut self, value: ResponsesFunctionToolCallStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn r#type(mut self, value: ResponsesFunctionToolCallType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesFunctionToolCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`arguments`](ResponsesFunctionToolCallBuilder::arguments)
    /// - [`call_id`](ResponsesFunctionToolCallBuilder::call_id)
    /// - [`name`](ResponsesFunctionToolCallBuilder::name)
    /// - [`r#type`](ResponsesFunctionToolCallBuilder::r#type)
    pub fn build(self) -> Result<ResponsesFunctionToolCall, BuildError> {
        Ok(ResponsesFunctionToolCall {
            arguments: self
                .arguments
                .ok_or_else(|| BuildError::missing_field("arguments"))?,
            call_id: self
                .call_id
                .ok_or_else(|| BuildError::missing_field("call_id"))?,
            caller: self.caller,
            id: self.id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            namespace: self.namespace,
            status: self.status,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
