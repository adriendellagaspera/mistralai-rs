#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum UserMessageRole {
    #[default]
    #[serde(rename = "user")]
    User,
}
