impl DocumentLibraryToolType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DocumentLibrary => "document_library",
        }
    }
}
