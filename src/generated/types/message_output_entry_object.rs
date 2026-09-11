#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MessageOutputEntryObject {
    #[default]
    #[serde(rename = "entry")]
    Entry,
}
