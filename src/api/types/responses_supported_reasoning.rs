pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResponsesSupportedReasoning {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<ResponsesSupportedReasoningEffort>,
}

impl ResponsesSupportedReasoning {
    pub fn builder() -> ResponsesSupportedReasoningBuilder {
        <ResponsesSupportedReasoningBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesSupportedReasoningBuilder {
    effort: Option<ResponsesSupportedReasoningEffort>,
}

impl ResponsesSupportedReasoningBuilder {
    pub fn effort(mut self, value: ResponsesSupportedReasoningEffort) -> Self {
        self.effort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesSupportedReasoning`].
    pub fn build(self) -> Result<ResponsesSupportedReasoning, BuildError> {
        Ok(ResponsesSupportedReasoning {
            effort: self.effort,
        })
    }
}
