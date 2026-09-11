#[derive(Debug, Clone)]
pub enum OutputContentChunks {
    TextChunk(TextChunk),
    ImageURLChunk(ImageURLChunk),
    ToolFileChunk(ToolFileChunk),
    DocumentURLChunk(DocumentURLChunk),
    ThinkChunk(ThinkChunk),
    ToolReferenceChunk(ToolReferenceChunk),
}
