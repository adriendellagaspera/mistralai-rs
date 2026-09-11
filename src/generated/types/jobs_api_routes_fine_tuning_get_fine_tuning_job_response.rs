#[derive(Debug, Clone)]
pub enum JobsApiRoutesFineTuningGetFineTuningJobResponse {
    CompletionDetailedJobOut(CompletionDetailedJobOut),
    ClassifierDetailedJobOut(ClassifierDetailedJobOut),
}
