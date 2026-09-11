#[derive(Debug, Clone)]
pub enum MessageInputContentChunksItemUnion {
    TextChunk(TextChunk),
    ImageURLChunk(ImageURLChunk),
    ToolFileChunk(ToolFileChunk),
    DocumentURLChunk(DocumentURLChunk),
    ThinkChunk(ThinkChunk),
}
