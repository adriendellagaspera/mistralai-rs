pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesFunctionCallOutput {
    #[serde(default)]
    pub call_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller: Option<ResponsesFunctionCallOutputCaller>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub output: ResponsesFunctionCallOutputOutput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponsesFunctionCallOutputStatus>,
    pub r#type: ResponsesFunctionCallOutputType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesFunctionCallOutput {
    pub fn builder() -> ResponsesFunctionCallOutputBuilder {
        <ResponsesFunctionCallOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesFunctionCallOutputBuilder {
    call_id: Option<String>,
    caller: Option<ResponsesFunctionCallOutputCaller>,
    id: Option<String>,
    output: Option<ResponsesFunctionCallOutputOutput>,
    status: Option<ResponsesFunctionCallOutputStatus>,
    r#type: Option<ResponsesFunctionCallOutputType>,
}

impl ResponsesFunctionCallOutputBuilder {
    pub fn call_id(mut self, value: impl Into<String>) -> Self {
        self.call_id = Some(value.into());
        self
    }

    pub fn caller(mut self, value: ResponsesFunctionCallOutputCaller) -> Self {
        self.caller = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn output(mut self, value: ResponsesFunctionCallOutputOutput) -> Self {
        self.output = Some(value);
        self
    }

    pub fn status(mut self, value: ResponsesFunctionCallOutputStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn r#type(mut self, value: ResponsesFunctionCallOutputType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesFunctionCallOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`call_id`](ResponsesFunctionCallOutputBuilder::call_id)
    /// - [`output`](ResponsesFunctionCallOutputBuilder::output)
    /// - [`r#type`](ResponsesFunctionCallOutputBuilder::r#type)
    pub fn build(self) -> Result<ResponsesFunctionCallOutput, BuildError> {
        Ok(ResponsesFunctionCallOutput {
            call_id: self
                .call_id
                .ok_or_else(|| BuildError::missing_field("call_id"))?,
            caller: self.caller,
            id: self.id,
            output: self
                .output
                .ok_or_else(|| BuildError::missing_field("output"))?,
            status: self.status,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
