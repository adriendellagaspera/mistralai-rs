///The type of job (`FT` for fine-tuning).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ClassifierJobOutJobType {
    #[default]
    #[serde(rename = "classifier")]
    Classifier,
}
