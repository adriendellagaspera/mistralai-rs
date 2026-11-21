pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateScorerRequest {
    pub config: LlmJudgeConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub kind: ScorerKind,
    #[serde(default)]
    pub name: String,
}

impl CreateScorerRequest {
    pub fn builder() -> CreateScorerRequestBuilder {
        <CreateScorerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateScorerRequestBuilder {
    config: Option<LlmJudgeConfig>,
    description: Option<String>,
    kind: Option<ScorerKind>,
    name: Option<String>,
}

impl CreateScorerRequestBuilder {
    pub fn config(mut self, value: LlmJudgeConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ScorerKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateScorerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](CreateScorerRequestBuilder::config)
    /// - [`kind`](CreateScorerRequestBuilder::kind)
    /// - [`name`](CreateScorerRequestBuilder::name)
    pub fn build(self) -> Result<CreateScorerRequest, BuildError> {
        Ok(CreateScorerRequest {
            config: self
                .config
                .ok_or_else(|| BuildError::missing_field("config"))?,
            description: self.description,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
