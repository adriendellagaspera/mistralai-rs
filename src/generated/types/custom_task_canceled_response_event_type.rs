///Event type discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CustomTaskCanceledResponseEventType {
    #[default]
    #[serde(rename = "CUSTOM_TASK_CANCELED")]
    CustomTaskCanceled,
}
