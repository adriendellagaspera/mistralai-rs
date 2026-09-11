///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CustomTaskTimedOutResponseEventType {
    #[default]
    #[serde(rename = "CUSTOM_TASK_TIMED_OUT")]
    CustomTaskTimedOut,
}
