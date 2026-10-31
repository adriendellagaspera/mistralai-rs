pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesCallerDirect {
    pub r#type: ResponsesCallerDirectType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesCallerDirect {
    pub fn builder() -> ResponsesCallerDirectBuilder {
        <ResponsesCallerDirectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesCallerDirectBuilder {
    r#type: Option<ResponsesCallerDirectType>,
}

impl ResponsesCallerDirectBuilder {
    pub fn r#type(mut self, value: ResponsesCallerDirectType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesCallerDirect`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ResponsesCallerDirectBuilder::r#type)
    pub fn build(self) -> Result<ResponsesCallerDirect, BuildError> {
        Ok(ResponsesCallerDirect {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
