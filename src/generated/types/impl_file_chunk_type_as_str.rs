impl FileChunkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::File => "file",
        }
    }
}
