#[derive(Debug, Clone)]
pub enum SystemMessageContentChunks {
    TextChunk(TextChunk),
    ThinkChunk(ThinkChunk),
}
