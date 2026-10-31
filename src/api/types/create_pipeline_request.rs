pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePipelineRequest {
    #[serde(default)]
    pub definitions: Vec<LlmJudgeReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub selectors: Vec<PipelineConfigSelector>,
}

impl CreatePipelineRequest {
    pub fn builder() -> CreatePipelineRequestBuilder {
        <CreatePipelineRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePipelineRequestBuilder {
    definitions: Option<Vec<LlmJudgeReference>>,
    description: Option<String>,
    enabled: Option<bool>,
    name: Option<String>,
    selectors: Option<Vec<PipelineConfigSelector>>,
}

impl CreatePipelineRequestBuilder {
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

    /// Consumes the builder and constructs a [`CreatePipelineRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`definitions`](CreatePipelineRequestBuilder::definitions)
    /// - [`name`](CreatePipelineRequestBuilder::name)
    /// - [`selectors`](CreatePipelineRequestBuilder::selectors)
    pub fn build(self) -> Result<CreatePipelineRequest, BuildError> {
        Ok(CreatePipelineRequest {
            definitions: self
                .definitions
                .ok_or_else(|| BuildError::missing_field("definitions"))?,
            description: self.description,
            enabled: self.enabled,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            selectors: self
                .selectors
                .ok_or_else(|| BuildError::missing_field("selectors"))?,
        })
    }
}
