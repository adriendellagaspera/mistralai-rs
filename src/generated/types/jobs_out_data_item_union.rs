#[derive(Debug, Clone)]
pub enum JobsOutDataItemUnion {
    CompletionJobOut(CompletionJobOut),
    ClassifierJobOut(ClassifierJobOut),
}
