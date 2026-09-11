///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ActivityTaskStartedResponseEventType {
    #[default]
    #[serde(rename = "ACTIVITY_TASK_STARTED")]
    ActivityTaskStarted,
}
