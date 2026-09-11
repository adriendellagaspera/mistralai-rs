///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CustomTaskFailedResponseEventType {
    #[default]
    #[serde(rename = "CUSTOM_TASK_FAILED")]
    CustomTaskFailed,
}
