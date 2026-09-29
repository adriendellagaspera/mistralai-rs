pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePromptVersionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplicated: Option<bool>,
}

impl CreatePromptVersionResponse {
    pub fn builder() -> CreatePromptVersionResponseBuilder {
        <CreatePromptVersionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePromptVersionResponseBuilder {
    version: Option<i64>,
    deduplicated: Option<bool>,
}

impl CreatePromptVersionResponseBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn deduplicated(mut self, value: bool) -> Self {
        self.deduplicated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePromptVersionResponse`].
    pub fn build(self) -> Result<CreatePromptVersionResponse, BuildError> {
        Ok(CreatePromptVersionResponse {
            version: self.version,
            deduplicated: self.deduplicated,
        })
    }
}
