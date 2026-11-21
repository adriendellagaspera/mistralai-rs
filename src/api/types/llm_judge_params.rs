pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmJudgeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
}

impl LlmJudgeParams {
    pub fn builder() -> LlmJudgeParamsBuilder {
        <LlmJudgeParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmJudgeParamsBuilder {
    max_tokens: Option<i64>,
    random_seed: Option<i64>,
    temperature: Option<f64>,
    top_p: Option<f64>,
}

impl LlmJudgeParamsBuilder {
    pub fn max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }

    pub fn random_seed(mut self, value: i64) -> Self {
        self.random_seed = Some(value);
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

    /// Consumes the builder and constructs a [`LlmJudgeParams`].
    pub fn build(self) -> Result<LlmJudgeParams, BuildError> {
        Ok(LlmJudgeParams {
            max_tokens: self.max_tokens,
            random_seed: self.random_seed,
            temperature: self.temperature,
            top_p: self.top_p,
        })
    }
}
