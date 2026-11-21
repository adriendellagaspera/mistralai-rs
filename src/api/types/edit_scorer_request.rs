pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EditScorerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<LlmJudgeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl EditScorerRequest {
    pub fn builder() -> EditScorerRequestBuilder {
        <EditScorerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditScorerRequestBuilder {
    config: Option<LlmJudgeConfig>,
    description: Option<String>,
    name: Option<String>,
}

impl EditScorerRequestBuilder {
    pub fn config(mut self, value: LlmJudgeConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EditScorerRequest`].
    pub fn build(self) -> Result<EditScorerRequest, BuildError> {
        Ok(EditScorerRequest {
            config: self.config,
            description: self.description,
            name: self.name,
        })
    }
}
