impl SampleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pretrain => "pretrain",
            Self::Instruct => "instruct",
            Self::BatchRequest => "batch_request",
            Self::BatchResult => "batch_result",
            Self::BatchError => "batch_error",
        }
    }
}
