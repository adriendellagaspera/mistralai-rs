pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListWorkflowEventResponse {
    /// List of workflow events.
    #[serde(default)]
    pub events: Vec<ListWorkflowEventResponseEventsItem>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl ListWorkflowEventResponse {
    pub fn builder() -> ListWorkflowEventResponseBuilder {
        <ListWorkflowEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWorkflowEventResponseBuilder {
    events: Option<Vec<ListWorkflowEventResponseEventsItem>>,
    next_cursor: Option<String>,
}

impl ListWorkflowEventResponseBuilder {
    pub fn events(mut self, value: Vec<ListWorkflowEventResponseEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListWorkflowEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](ListWorkflowEventResponseBuilder::events)
    pub fn build(self) -> Result<ListWorkflowEventResponse, BuildError> {
        Ok(ListWorkflowEventResponse {
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            next_cursor: self.next_cursor,
        })
    }
}
