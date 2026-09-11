///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ActivityTaskRetryingResponseEventType {
    #[default]
    #[serde(rename = "ACTIVITY_TASK_RETRYING")]
    ActivityTaskRetrying,
}
