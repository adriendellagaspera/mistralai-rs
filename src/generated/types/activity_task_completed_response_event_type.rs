///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ActivityTaskCompletedResponseEventType {
    #[default]
    #[serde(rename = "ACTIVITY_TASK_COMPLETED")]
    ActivityTaskCompleted,
}
