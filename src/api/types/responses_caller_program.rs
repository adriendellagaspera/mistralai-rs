pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesCallerProgram {
    #[serde(default)]
    pub caller_id: String,
    pub r#type: ResponsesCallerProgramType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesCallerProgram {
    pub fn builder() -> ResponsesCallerProgramBuilder {
        <ResponsesCallerProgramBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesCallerProgramBuilder {
    caller_id: Option<String>,
    r#type: Option<ResponsesCallerProgramType>,
}

impl ResponsesCallerProgramBuilder {
    pub fn caller_id(mut self, value: impl Into<String>) -> Self {
        self.caller_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ResponsesCallerProgramType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesCallerProgram`].
    /// This method will fail if any of the following fields are not set:
    /// - [`caller_id`](ResponsesCallerProgramBuilder::caller_id)
    /// - [`r#type`](ResponsesCallerProgramBuilder::r#type)
    pub fn build(self) -> Result<ResponsesCallerProgram, BuildError> {
        Ok(ResponsesCallerProgram {
            caller_id: self
                .caller_id
                .ok_or_else(|| BuildError::missing_field("caller_id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
