#[derive(Debug, Clone)]
pub enum JobsApiRoutesFineTuningUpdateFineTunedModelResponse {
    CompletionFTModelOut(CompletionFTModelOut),
    ClassifierFTModelOut(ClassifierFTModelOut),
}
