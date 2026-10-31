pub use crate::prelude::*;

/// The fully materialized LLM judge behavior stored in a pipeline config.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JudgeDefinitionV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i64>,
    pub result_spec: EvaluationResultSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_cache: Option<bool>,
}

impl JudgeDefinitionV1 {
    pub fn builder() -> JudgeDefinitionV1Builder {
        <JudgeDefinitionV1Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgeDefinitionV1Builder {
    mapping: Option<HashMap<String, String>>,
    max_tokens: Option<i64>,
    model: Option<String>,
    prompt: Option<String>,
    random_seed: Option<i64>,
    result_spec: Option<EvaluationResultSpec>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    use_cache: Option<bool>,
}

impl JudgeDefinitionV1Builder {
    pub fn mapping(mut self, value: HashMap<String, String>) -> Self {
        self.mapping = Some(value);
        self
    }

    pub fn max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn random_seed(mut self, value: i64) -> Self {
        self.random_seed = Some(value);
        self
    }

    pub fn result_spec(mut self, value: EvaluationResultSpec) -> Self {
        self.result_spec = Some(value);
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

    pub fn use_cache(mut self, value: bool) -> Self {
        self.use_cache = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JudgeDefinitionV1`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](JudgeDefinitionV1Builder::model)
    /// - [`prompt`](JudgeDefinitionV1Builder::prompt)
    /// - [`result_spec`](JudgeDefinitionV1Builder::result_spec)
    pub fn build(self) -> Result<JudgeDefinitionV1, BuildError> {
        Ok(JudgeDefinitionV1 {
            mapping: self.mapping,
            max_tokens: self.max_tokens,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
            random_seed: self.random_seed,
            result_spec: self
                .result_spec
                .ok_or_else(|| BuildError::missing_field("result_spec"))?,
            temperature: self.temperature,
            top_p: self.top_p,
            use_cache: self.use_cache,
        })
    }
}
