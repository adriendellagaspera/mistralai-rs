pub use crate::prelude::*;

/// White-listed arguments from the completion API
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompletionArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<CompletionArgsStop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction: Option<Prediction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoiceEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
}

impl CompletionArgs {
    pub fn builder() -> CompletionArgsBuilder {
        <CompletionArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompletionArgsBuilder {
    stop: Option<CompletionArgsStop>,
    presence_penalty: Option<f64>,
    frequency_penalty: Option<f64>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    max_tokens: Option<i64>,
    random_seed: Option<i64>,
    prediction: Option<Prediction>,
    response_format: Option<ResponseFormat>,
    tool_choice: Option<ToolChoiceEnum>,
    reasoning_effort: Option<ReasoningEffort>,
}

impl CompletionArgsBuilder {
    pub fn stop(mut self, value: CompletionArgsStop) -> Self {
        self.stop = Some(value);
        self
    }

    pub fn presence_penalty(mut self, value: f64) -> Self {
        self.presence_penalty = Some(value);
        self
    }

    pub fn frequency_penalty(mut self, value: f64) -> Self {
        self.frequency_penalty = Some(value);
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn top_p(mut self, value: f64) -> Self {
        self.top_p = Some(value);
        self
    }

    pub fn max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }

    pub fn random_seed(mut self, value: i64) -> Self {
        self.random_seed = Some(value);
        self
    }

    pub fn prediction(mut self, value: Prediction) -> Self {
        self.prediction = Some(value);
        self
    }

    pub fn response_format(mut self, value: ResponseFormat) -> Self {
        self.response_format = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: ToolChoiceEnum) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn reasoning_effort(mut self, value: ReasoningEffort) -> Self {
        self.reasoning_effort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompletionArgs`].
    pub fn build(self) -> Result<CompletionArgs, BuildError> {
        Ok(CompletionArgs {
            stop: self.stop,
            presence_penalty: self.presence_penalty,
            frequency_penalty: self.frequency_penalty,
            temperature: self.temperature,
            top_p: self.top_p,
            max_tokens: self.max_tokens,
            random_seed: self.random_seed,
            prediction: self.prediction,
            response_format: self.response_format,
            tool_choice: self.tool_choice,
            reasoning_effort: self.reasoning_effort,
        })
    }
}
