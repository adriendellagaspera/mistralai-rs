pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetSpanEvaluation {
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub customer_id: String,
    #[serde(default)]
    pub evaluation_name: String,
    #[serde(default)]
    pub explanation: String,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub response_id: String,
    #[serde(default)]
    pub score_label: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub score_value: f64,
    #[serde(default)]
    pub span_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    #[serde(default)]
    pub trace_id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub workspace_id: String,
}

impl GetSpanEvaluation {
    pub fn builder() -> GetSpanEvaluationBuilder {
        <GetSpanEvaluationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSpanEvaluationBuilder {
    conversation_id: Option<String>,
    customer_id: Option<String>,
    evaluation_name: Option<String>,
    explanation: Option<String>,
    metadata: Option<HashMap<String, String>>,
    organization_id: Option<String>,
    response_id: Option<String>,
    score_label: Option<String>,
    score_value: Option<f64>,
    span_id: Option<String>,
    timestamp: Option<DateTime<FixedOffset>>,
    trace_id: Option<String>,
    user_id: Option<String>,
    workspace_id: Option<String>,
}

impl GetSpanEvaluationBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn evaluation_name(mut self, value: impl Into<String>) -> Self {
        self.evaluation_name = Some(value.into());
        self
    }

    pub fn explanation(mut self, value: impl Into<String>) -> Self {
        self.explanation = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn response_id(mut self, value: impl Into<String>) -> Self {
        self.response_id = Some(value.into());
        self
    }

    pub fn score_label(mut self, value: impl Into<String>) -> Self {
        self.score_label = Some(value.into());
        self
    }

    pub fn score_value(mut self, value: f64) -> Self {
        self.score_value = Some(value);
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSpanEvaluation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](GetSpanEvaluationBuilder::conversation_id)
    /// - [`customer_id`](GetSpanEvaluationBuilder::customer_id)
    /// - [`evaluation_name`](GetSpanEvaluationBuilder::evaluation_name)
    /// - [`explanation`](GetSpanEvaluationBuilder::explanation)
    /// - [`metadata`](GetSpanEvaluationBuilder::metadata)
    /// - [`organization_id`](GetSpanEvaluationBuilder::organization_id)
    /// - [`response_id`](GetSpanEvaluationBuilder::response_id)
    /// - [`score_label`](GetSpanEvaluationBuilder::score_label)
    /// - [`score_value`](GetSpanEvaluationBuilder::score_value)
    /// - [`span_id`](GetSpanEvaluationBuilder::span_id)
    /// - [`timestamp`](GetSpanEvaluationBuilder::timestamp)
    /// - [`trace_id`](GetSpanEvaluationBuilder::trace_id)
    /// - [`user_id`](GetSpanEvaluationBuilder::user_id)
    /// - [`workspace_id`](GetSpanEvaluationBuilder::workspace_id)
    pub fn build(self) -> Result<GetSpanEvaluation, BuildError> {
        Ok(GetSpanEvaluation {
            conversation_id: self
                .conversation_id
                .ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            evaluation_name: self
                .evaluation_name
                .ok_or_else(|| BuildError::missing_field("evaluation_name"))?,
            explanation: self
                .explanation
                .ok_or_else(|| BuildError::missing_field("explanation"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            organization_id: self
                .organization_id
                .ok_or_else(|| BuildError::missing_field("organization_id"))?,
            response_id: self
                .response_id
                .ok_or_else(|| BuildError::missing_field("response_id"))?,
            score_label: self
                .score_label
                .ok_or_else(|| BuildError::missing_field("score_label"))?,
            score_value: self
                .score_value
                .ok_or_else(|| BuildError::missing_field("score_value"))?,
            span_id: self
                .span_id
                .ok_or_else(|| BuildError::missing_field("span_id"))?,
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            trace_id: self
                .trace_id
                .ok_or_else(|| BuildError::missing_field("trace_id"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
