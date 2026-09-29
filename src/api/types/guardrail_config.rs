pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GuardrailConfig {
    /// If true, return HTTP 403 and block request in the event of a server-side error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_on_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_llm_v1: Option<ModerationLlmv1Config>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_llm_v2: Option<ModerationLlmv2Config>,
}

impl GuardrailConfig {
    pub fn builder() -> GuardrailConfigBuilder {
        <GuardrailConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GuardrailConfigBuilder {
    block_on_error: Option<bool>,
    moderation_llm_v1: Option<ModerationLlmv1Config>,
    moderation_llm_v2: Option<ModerationLlmv2Config>,
}

impl GuardrailConfigBuilder {
    pub fn block_on_error(mut self, value: bool) -> Self {
        self.block_on_error = Some(value);
        self
    }

    pub fn moderation_llm_v1(mut self, value: ModerationLlmv1Config) -> Self {
        self.moderation_llm_v1 = Some(value);
        self
    }

    pub fn moderation_llm_v2(mut self, value: ModerationLlmv2Config) -> Self {
        self.moderation_llm_v2 = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GuardrailConfig`].
    pub fn build(self) -> Result<GuardrailConfig, BuildError> {
        Ok(GuardrailConfig {
            block_on_error: self.block_on_error,
            moderation_llm_v1: self.moderation_llm_v1,
            moderation_llm_v2: self.moderation_llm_v2,
        })
    }
}
