///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CustomTaskStartedResponseEventType {
    #[default]
    #[serde(rename = "CUSTOM_TASK_STARTED")]
    CustomTaskStarted,
}
