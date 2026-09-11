///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ActivityTaskFailedResponseEventType {
    #[default]
    #[serde(rename = "ACTIVITY_TASK_FAILED")]
    ActivityTaskFailed,
}
