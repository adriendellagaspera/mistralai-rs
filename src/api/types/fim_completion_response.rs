pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FimCompletionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default)]
    pub choices: Vec<ChatCompletionChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageInfo>,
}

impl FimCompletionResponse {
    pub fn builder() -> FimCompletionResponseBuilder {
        <FimCompletionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FimCompletionResponseBuilder {
    model: Option<String>,
    choices: Option<Vec<ChatCompletionChoice>>,
    created: Option<i64>,
    id: Option<String>,
    object: Option<String>,
    usage: Option<UsageInfo>,
}

impl FimCompletionResponseBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn choices(mut self, value: Vec<ChatCompletionChoice>) -> Self {
        self.choices = Some(value);
        self
    }

    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn usage(mut self, value: UsageInfo) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FimCompletionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`choices`](FimCompletionResponseBuilder::choices)
    pub fn build(self) -> Result<FimCompletionResponse, BuildError> {
        Ok(FimCompletionResponse {
            model: self.model,
            choices: self
                .choices
                .ok_or_else(|| BuildError::missing_field("choices"))?,
            created: self.created,
            id: self.id,
            object: self.object,
            usage: self.usage,
        })
    }
}
