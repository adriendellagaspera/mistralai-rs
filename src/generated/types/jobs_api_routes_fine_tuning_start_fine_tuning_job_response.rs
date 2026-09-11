#[derive(Debug, Clone)]
pub enum JobsApiRoutesFineTuningStartFineTuningJobResponse {
    CompletionDetailedJobOut(CompletionDetailedJobOut),
    ClassifierDetailedJobOut(ClassifierDetailedJobOut),
}
