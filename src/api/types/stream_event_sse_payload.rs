pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamEventSsePayload {
    #[serde(default)]
    pub stream: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    pub data: StreamEventSsePayloadData,
    #[serde(default)]
    pub workflow_context: StreamEventWorkflowContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub broker_sequence: i64,
}

impl StreamEventSsePayload {
    pub fn builder() -> StreamEventSsePayloadBuilder {
        <StreamEventSsePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamEventSsePayloadBuilder {
    stream: Option<String>,
    timestamp: Option<DateTime<FixedOffset>>,
    data: Option<StreamEventSsePayloadData>,
    workflow_context: Option<StreamEventWorkflowContext>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    broker_sequence: Option<i64>,
}

impl StreamEventSsePayloadBuilder {
    pub fn stream(mut self, value: impl Into<String>) -> Self {
        self.stream = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn data(mut self, value: StreamEventSsePayloadData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn workflow_context(mut self, value: StreamEventWorkflowContext) -> Self {
        self.workflow_context = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn broker_sequence(mut self, value: i64) -> Self {
        self.broker_sequence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StreamEventSsePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`stream`](StreamEventSsePayloadBuilder::stream)
    /// - [`data`](StreamEventSsePayloadBuilder::data)
    /// - [`workflow_context`](StreamEventSsePayloadBuilder::workflow_context)
    /// - [`broker_sequence`](StreamEventSsePayloadBuilder::broker_sequence)
    pub fn build(self) -> Result<StreamEventSsePayload, BuildError> {
        Ok(StreamEventSsePayload {
            stream: self
                .stream
                .ok_or_else(|| BuildError::missing_field("stream"))?,
            timestamp: self.timestamp,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            workflow_context: self
                .workflow_context
                .ok_or_else(|| BuildError::missing_field("workflow_context"))?,
            metadata: self.metadata,
            broker_sequence: self
                .broker_sequence
                .ok_or_else(|| BuildError::missing_field("broker_sequence"))?,
        })
    }
}
