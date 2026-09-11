#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum AgentHandoffEntryObject {
    #[default]
    #[serde(rename = "entry")]
    Entry,
}
