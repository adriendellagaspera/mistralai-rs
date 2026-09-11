///Attributes for activity task retrying events.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivityTaskRetryingAttributes {
    ///The registered name of the activity being executed.
    pub activity_name: String,
    ///The attempt number that failed (1-indexed).
    pub attempt: i64,
    pub failure: Failure,
    ///Unique identifier for the activity task within the workflow.
    pub task_id: String,
}
