impl ApiEndpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::V1ChatCompletions => "/v1/chat/completions",
            Self::V1Embeddings => "/v1/embeddings",
            Self::V1FimCompletions => "/v1/fim/completions",
            Self::V1Moderations => "/v1/moderations",
            Self::V1ChatModerations => "/v1/chat/moderations",
            Self::V1Ocr => "/v1/ocr",
            Self::V1Classifications => "/v1/classifications",
            Self::V1ChatClassifications => "/v1/chat/classifications",
            Self::V1Conversations => "/v1/conversations",
            Self::V1AudioTranscriptions => "/v1/audio/transcriptions",
        }
    }
}
