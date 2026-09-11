#[derive(Debug, Clone)]
pub enum JobsApiRoutesFineTuningCancelFineTuningJobResponse {
    CompletionDetailedJobOut(CompletionDetailedJobOut),
    ClassifierDetailedJobOut(ClassifierDetailedJobOut),
}
