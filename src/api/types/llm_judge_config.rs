pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LlmJudgeConfig {
    #[serde(default)]
    pub model: String,
    pub output_spec: EvaluationResultSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<LlmJudgeParams>,
    #[serde(default)]
    pub prompt_template: String,
}

impl LlmJudgeConfig {
    pub fn builder() -> LlmJudgeConfigBuilder {
        <LlmJudgeConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmJudgeConfigBuilder {
    model: Option<String>,
    output_spec: Option<EvaluationResultSpec>,
    params: Option<LlmJudgeParams>,
    prompt_template: Option<String>,
}

impl LlmJudgeConfigBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn output_spec(mut self, value: EvaluationResultSpec) -> Self {
        self.output_spec = Some(value);
        self
    }

    pub fn params(mut self, value: LlmJudgeParams) -> Self {
        self.params = Some(value);
        self
    }

    pub fn prompt_template(mut self, value: impl Into<String>) -> Self {
        self.prompt_template = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LlmJudgeConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](LlmJudgeConfigBuilder::model)
    /// - [`output_spec`](LlmJudgeConfigBuilder::output_spec)
    /// - [`prompt_template`](LlmJudgeConfigBuilder::prompt_template)
    pub fn build(self) -> Result<LlmJudgeConfig, BuildError> {
        Ok(LlmJudgeConfig {
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            output_spec: self
                .output_spec
                .ok_or_else(|| BuildError::missing_field("output_spec"))?,
            params: self.params,
            prompt_template: self
                .prompt_template
                .ok_or_else(|| BuildError::missing_field("prompt_template"))?,
        })
    }
}
