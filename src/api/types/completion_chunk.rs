pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompletionChunk {
    #[serde(default)]
    pub choices: Vec<CompletionResponseStreamChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageInfo>,
}

impl CompletionChunk {
    pub fn builder() -> CompletionChunkBuilder {
        <CompletionChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompletionChunkBuilder {
    choices: Option<Vec<CompletionResponseStreamChoice>>,
    created: Option<i64>,
    id: Option<String>,
    model: Option<String>,
    object: Option<String>,
    usage: Option<UsageInfo>,
}

impl CompletionChunkBuilder {
    pub fn choices(mut self, value: Vec<CompletionResponseStreamChoice>) -> Self {
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

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
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

    /// Consumes the builder and constructs a [`CompletionChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`choices`](CompletionChunkBuilder::choices)
    /// - [`id`](CompletionChunkBuilder::id)
    /// - [`model`](CompletionChunkBuilder::model)
    pub fn build(self) -> Result<CompletionChunk, BuildError> {
        Ok(CompletionChunk {
            choices: self
                .choices
                .ok_or_else(|| BuildError::missing_field("choices"))?,
            created: self.created,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            object: self.object,
            usage: self.usage,
        })
    }
}
