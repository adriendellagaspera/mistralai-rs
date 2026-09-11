///Represents an error or exception that occurred during execution.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Failure {
    ///A human-readable description of the failure.
    pub message: String,
}
