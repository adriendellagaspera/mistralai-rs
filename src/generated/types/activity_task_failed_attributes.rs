///Attributes for activity task failed events (final failure after all retries).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActivityTaskFailedAttributes {
    ///The registered name of the activity being executed.
    pub activity_name: String,
    ///The final attempt number that failed (1-indexed).
    pub attempt: i64,
    pub failure: Failure,
    ///Unique identifier for the activity task within the workflow.
    pub task_id: String,
}
