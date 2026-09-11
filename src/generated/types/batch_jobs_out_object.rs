#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BatchJobsOutObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
