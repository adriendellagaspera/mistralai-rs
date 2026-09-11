#[derive(Debug, Clone)]
pub enum MessageOutputContentChunksItemUnion {
    TextChunk(TextChunk),
    ImageURLChunk(ImageURLChunk),
    ToolFileChunk(ToolFileChunk),
    DocumentURLChunk(DocumentURLChunk),
    ThinkChunk(ThinkChunk),
    ToolReferenceChunk(ToolReferenceChunk),
}
