#[derive(Debug, Clone)]
pub enum JobInHyperparameters {
    CompletionTrainingParametersIn(CompletionTrainingParametersIn),
    ClassifierTrainingParametersIn(ClassifierTrainingParametersIn),
}
