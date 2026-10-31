pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesFunctionCallOutputCallerDirect {
    pub r#type: ResponsesFunctionCallOutputCallerDirectType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesFunctionCallOutputCallerDirect {
    pub fn builder() -> ResponsesFunctionCallOutputCallerDirectBuilder {
        <ResponsesFunctionCallOutputCallerDirectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesFunctionCallOutputCallerDirectBuilder {
    r#type: Option<ResponsesFunctionCallOutputCallerDirectType>,
}

impl ResponsesFunctionCallOutputCallerDirectBuilder {
    pub fn r#type(mut self, value: ResponsesFunctionCallOutputCallerDirectType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesFunctionCallOutputCallerDirect`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ResponsesFunctionCallOutputCallerDirectBuilder::r#type)
    pub fn build(self) -> Result<ResponsesFunctionCallOutputCallerDirect, BuildError> {
        Ok(ResponsesFunctionCallOutputCallerDirect {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
