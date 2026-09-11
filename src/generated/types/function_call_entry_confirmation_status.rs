#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FunctionCallEntryConfirmationStatus {
    #[default]
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "denied")]
    Denied,
}
