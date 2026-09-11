#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MessageInputEntryObject {
    #[default]
    #[serde(rename = "entry")]
    Entry,
}
