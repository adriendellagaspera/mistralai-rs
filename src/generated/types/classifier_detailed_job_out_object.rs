#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ClassifierDetailedJobOutObject {
    #[default]
    #[serde(rename = "job")]
    Job,
}
