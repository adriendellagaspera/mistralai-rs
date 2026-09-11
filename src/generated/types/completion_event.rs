#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompletionEvent {
    pub data: CompletionChunk,
}
