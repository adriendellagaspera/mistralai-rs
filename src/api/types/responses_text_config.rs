pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesTextConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ResponsesTextConfigFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<ResponsesTextConfigVerbosity>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesTextConfig {
    pub fn builder() -> ResponsesTextConfigBuilder {
        <ResponsesTextConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesTextConfigBuilder {
    format: Option<ResponsesTextConfigFormat>,
    verbosity: Option<ResponsesTextConfigVerbosity>,
}

impl ResponsesTextConfigBuilder {
    pub fn format(mut self, value: ResponsesTextConfigFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn verbosity(mut self, value: ResponsesTextConfigVerbosity) -> Self {
        self.verbosity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesTextConfig`].
    pub fn build(self) -> Result<ResponsesTextConfig, BuildError> {
        Ok(ResponsesTextConfig {
            format: self.format,
            verbosity: self.verbosity,
            extra: Default::default(),
        })
    }
}
