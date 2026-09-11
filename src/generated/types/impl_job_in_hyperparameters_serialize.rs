impl Serialize for JobInHyperparameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::CompletionTrainingParametersIn(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::ClassifierTrainingParametersIn(value) => {
                serde::Serialize::serialize(value, serializer)
            }
        }
    }
}
