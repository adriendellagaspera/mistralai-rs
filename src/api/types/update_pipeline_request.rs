pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdatePipelineRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<Vec<LlmJudgeReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<Vec<PipelineConfigSelector>>,
}

impl UpdatePipelineRequest {
    pub fn builder() -> UpdatePipelineRequestBuilder {
        <UpdatePipelineRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePipelineRequestBuilder {
    definitions: Option<Vec<LlmJudgeReference>>,
    description: Option<String>,
    enabled: Option<bool>,
    name: Option<String>,
    selectors: Option<Vec<PipelineConfigSelector>>,
}

impl UpdatePipelineRequestBuilder {
    pub fn definitions(mut self, value: Vec<LlmJudgeReference>) -> Self {
        self.definitions = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn selectors(mut self, value: Vec<PipelineConfigSelector>) -> Self {
        self.selectors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePipelineRequest`].
    pub fn build(self) -> Result<UpdatePipelineRequest, BuildError> {
        Ok(UpdatePipelineRequest {
            definitions: self.definitions,
            description: self.description,
            enabled: self.enabled,
            name: self.name,
            selectors: self.selectors,
        })
    }
}
