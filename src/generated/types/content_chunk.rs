#[derive(Debug, Clone)]
pub enum ContentChunk {
    TextChunk(TextChunk),
    ImageURLChunk(ImageURLChunk),
    DocumentURLChunk(DocumentURLChunk),
    ReferenceChunk(ReferenceChunk),
    FileChunk(FileChunk),
    ThinkChunk(ThinkChunk),
    AudioChunk(AudioChunk),
}
