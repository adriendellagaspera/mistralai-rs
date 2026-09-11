impl ConversationSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Explorer => "EXPLORER",
            Self::UploadedFile => "UPLOADED_FILE",
            Self::DirectInput => "DIRECT_INPUT",
            Self::Playground => "PLAYGROUND",
        }
    }
}
