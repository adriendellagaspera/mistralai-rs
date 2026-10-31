pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesFunctionCallOutputCallerProgram {
    #[serde(default)]
    pub caller_id: String,
    pub r#type: ResponsesFunctionCallOutputCallerProgramType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesFunctionCallOutputCallerProgram {
    pub fn builder() -> ResponsesFunctionCallOutputCallerProgramBuilder {
        <ResponsesFunctionCallOutputCallerProgramBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesFunctionCallOutputCallerProgramBuilder {
    caller_id: Option<String>,
    r#type: Option<ResponsesFunctionCallOutputCallerProgramType>,
}

impl ResponsesFunctionCallOutputCallerProgramBuilder {
    pub fn caller_id(mut self, value: impl Into<String>) -> Self {
        self.caller_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ResponsesFunctionCallOutputCallerProgramType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesFunctionCallOutputCallerProgram`].
    /// This method will fail if any of the following fields are not set:
    /// - [`caller_id`](ResponsesFunctionCallOutputCallerProgramBuilder::caller_id)
    /// - [`r#type`](ResponsesFunctionCallOutputCallerProgramBuilder::r#type)
    pub fn build(self) -> Result<ResponsesFunctionCallOutputCallerProgram, BuildError> {
        Ok(ResponsesFunctionCallOutputCallerProgram {
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
