#[derive(Debug, Clone)]
pub enum MessageEntriesItemUnion {
    MessageInputEntry(MessageInputEntry),
    MessageOutputEntry(MessageOutputEntry),
}
