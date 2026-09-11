///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CustomTaskCompletedResponseEventType {
    #[default]
    #[serde(rename = "CUSTOM_TASK_COMPLETED")]
    CustomTaskCompleted,
}
