///The current state or incremental update for the task.
#[derive(Debug, Clone)]
pub enum CustomTaskInProgressAttributesResponsePayload {
    JSONPayloadResponse(JSONPayloadResponse),
    JSONPatchPayloadResponse(JSONPatchPayloadResponse),
}
