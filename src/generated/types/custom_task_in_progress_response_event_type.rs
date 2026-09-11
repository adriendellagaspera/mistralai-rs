///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CustomTaskInProgressResponseEventType {
    #[default]
    #[serde(rename = "CUSTOM_TASK_IN_PROGRESS")]
    CustomTaskInProgress,
}
