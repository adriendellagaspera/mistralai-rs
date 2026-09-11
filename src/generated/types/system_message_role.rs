#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum SystemMessageRole {
    #[default]
    #[serde(rename = "system")]
    System,
}
