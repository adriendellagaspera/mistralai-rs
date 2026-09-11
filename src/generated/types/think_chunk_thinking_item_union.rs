#[derive(Debug, Clone)]
pub enum ThinkChunkThinkingItemUnion {
    TextChunk(TextChunk),
    ToolReferenceChunk(ToolReferenceChunk),
    ReferenceChunk(ReferenceChunk),
}
