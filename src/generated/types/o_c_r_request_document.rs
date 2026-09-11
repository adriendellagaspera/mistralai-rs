///Document to run OCR on
#[derive(Debug, Clone)]
pub enum OCRRequestDocument {
    FileChunk(FileChunk),
    DocumentURLChunk(DocumentURLChunk),
    ImageURLChunk(ImageURLChunk),
}
