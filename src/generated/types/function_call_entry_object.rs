#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FunctionCallEntryObject {
    #[default]
    #[serde(rename = "entry")]
    Entry,
}
