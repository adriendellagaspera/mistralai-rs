#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum JobsOutObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
