pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaTriggersListQueryRequest {
    /// Filter by handler type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<TriggerHandler>,
    /// Filter by primitive ID (workflow, agent, or app)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primitive_id: Option<String>,
    /// Filter by event source ID (webhook id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_id: Option<String>,
}

impl BetaTriggersListQueryRequest {
    pub fn builder() -> BetaTriggersListQueryRequestBuilder {
        <BetaTriggersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaTriggersListQueryRequestBuilder {
    handler: Option<TriggerHandler>,
    primitive_id: Option<String>,
    event_source_id: Option<String>,
}

impl BetaTriggersListQueryRequestBuilder {
    pub fn handler(mut self, value: TriggerHandler) -> Self {
        self.handler = Some(value);
        self
    }

    pub fn primitive_id(mut self, value: impl Into<String>) -> Self {
        self.primitive_id = Some(value.into());
        self
    }

    pub fn event_source_id(mut self, value: impl Into<String>) -> Self {
        self.event_source_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BetaTriggersListQueryRequest`].
    pub fn build(self) -> Result<BetaTriggersListQueryRequest, BuildError> {
        Ok(BetaTriggersListQueryRequest {
            handler: self.handler,
            primitive_id: self.primitive_id,
            event_source_id: self.event_source_id,
        })
    }
}
