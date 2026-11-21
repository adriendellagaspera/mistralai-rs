pub use crate::prelude::*;

/// Query parameters for list_trigger_events_v1_triggers__trigger_id__events_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTriggerEventsV1TriggersTriggerIdEventsGetQueryRequest {
    /// Maximum number of events to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListTriggerEventsV1TriggersTriggerIdEventsGetQueryRequest {
    pub fn builder() -> ListTriggerEventsV1TriggersTriggerIdEventsGetQueryRequestBuilder {
        <ListTriggerEventsV1TriggersTriggerIdEventsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTriggerEventsV1TriggersTriggerIdEventsGetQueryRequestBuilder {
    limit: Option<i64>,
}

impl ListTriggerEventsV1TriggersTriggerIdEventsGetQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTriggerEventsV1TriggersTriggerIdEventsGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<ListTriggerEventsV1TriggersTriggerIdEventsGetQueryRequest, BuildError> {
        Ok(ListTriggerEventsV1TriggersTriggerIdEventsGetQueryRequest { limit: self.limit })
    }
}
