pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationUsageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<HashMap<String, Option<i64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i64>,
}

impl ConversationUsageInfo {
    pub fn builder() -> ConversationUsageInfoBuilder {
        <ConversationUsageInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationUsageInfoBuilder {
    completion_tokens: Option<i64>,
    connector_tokens: Option<i64>,
    connectors: Option<HashMap<String, Option<i64>>>,
    prompt_tokens: Option<i64>,
    total_tokens: Option<i64>,
}

impl ConversationUsageInfoBuilder {
    pub fn completion_tokens(mut self, value: i64) -> Self {
        self.completion_tokens = Some(value);
        self
    }

    pub fn connector_tokens(mut self, value: i64) -> Self {
        self.connector_tokens = Some(value);
        self
    }

    pub fn connectors(mut self, value: HashMap<String, Option<i64>>) -> Self {
        self.connectors = Some(value);
        self
    }

    pub fn prompt_tokens(mut self, value: i64) -> Self {
        self.prompt_tokens = Some(value);
        self
    }

    pub fn total_tokens(mut self, value: i64) -> Self {
        self.total_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationUsageInfo`].
    pub fn build(self) -> Result<ConversationUsageInfo, BuildError> {
        Ok(ConversationUsageInfo {
            completion_tokens: self.completion_tokens,
            connector_tokens: self.connector_tokens,
            connectors: self.connectors,
            prompt_tokens: self.prompt_tokens,
            total_tokens: self.total_tokens,
        })
    }
}
