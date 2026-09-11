#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FunctionResultEntryObject {
    #[default]
    #[serde(rename = "entry")]
    Entry,
}
