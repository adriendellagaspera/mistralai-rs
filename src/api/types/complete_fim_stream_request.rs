pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompleteFimStreamRequest {
    /// ID of the model with FIM to use.
    #[serde(default)]
    pub model: String,
    /// What sampling temperature to use, we recommend between 0.0 and 0.7. Higher values like 0.7 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. We generally recommend altering this or `top_p` but not both. The default value varies depending on the model you are targeting. Call the `/models` endpoint to retrieve the appropriate value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Nucleus sampling, where the model considers the results of the tokens with `top_p` probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered. We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// The maximum number of tokens to generate in the completion. The token count of your prompt plus `max_tokens` cannot exceed the model's context length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    /// Whether to stream back partial progress. If set, tokens will be sent as data-only server-side events as they become available, with the stream terminated by a data: [DONE] message. Otherwise, the server will hold the request open until the timeout or until completion, with the response containing the full result as JSON.
    pub stream: bool,
    /// Stop generation if this token is detected. Or if one of these tokens is detected when providing an array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<CompleteFimStreamRequestStop>,
    /// The seed to use for random sampling. If set, different calls will generate deterministic results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The text/code to complete.
    #[serde(default)]
    pub prompt: String,
    /// Optional text/code that adds more context for the model. When given a `prompt` and a `suffix` the model will fill what is between them. When `suffix` is not provided, the model will simply execute completion starting with `prompt`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// The minimum number of tokens to generate in the completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
}

impl CompleteFimStreamRequest {
    pub fn builder() -> CompleteFimStreamRequestBuilder {
        <CompleteFimStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteFimStreamRequestBuilder {
    model: Option<String>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    max_tokens: Option<i64>,
    stream: Option<bool>,
    stop: Option<CompleteFimStreamRequestStop>,
    random_seed: Option<i64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    prompt: Option<String>,
    suffix: Option<String>,
    min_tokens: Option<i64>,
    prompt_cache_key: Option<String>,
}

impl CompleteFimStreamRequestBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
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

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
        self
    }

    pub fn stop(mut self, value: CompleteFimStreamRequestStop) -> Self {
        self.stop = Some(value);
        self
    }

    pub fn random_seed(mut self, value: i64) -> Self {
        self.random_seed = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn suffix(mut self, value: impl Into<String>) -> Self {
        self.suffix = Some(value.into());
        self
    }

    pub fn min_tokens(mut self, value: i64) -> Self {
        self.min_tokens = Some(value);
        self
    }

    pub fn prompt_cache_key(mut self, value: impl Into<String>) -> Self {
        self.prompt_cache_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompleteFimStreamRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](CompleteFimStreamRequestBuilder::model)
    /// - [`stream`](CompleteFimStreamRequestBuilder::stream)
    /// - [`prompt`](CompleteFimStreamRequestBuilder::prompt)
    pub fn build(self) -> Result<CompleteFimStreamRequest, BuildError> {
        Ok(CompleteFimStreamRequest {
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            temperature: self.temperature,
            top_p: self.top_p,
            max_tokens: self.max_tokens,
            stream: self
                .stream
                .ok_or_else(|| BuildError::missing_field("stream"))?,
            stop: self.stop,
            random_seed: self.random_seed,
            metadata: self.metadata,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
            suffix: self.suffix,
            min_tokens: self.min_tokens,
            prompt_cache_key: self.prompt_cache_key,
        })
    }
}
