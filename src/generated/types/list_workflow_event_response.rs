#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListWorkflowEventResponse {
    ///List of workflow events.
    pub events: Vec<ListWorkflowEventResponseEventsItemUnion>,
    ///Cursor for pagination.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub next_cursor: Option<Option<String>>,
}
