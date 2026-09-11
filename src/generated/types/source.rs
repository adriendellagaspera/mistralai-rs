#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum Source {
    #[default]
    #[serde(rename = "upload")]
    Upload,
    #[serde(rename = "repository")]
    Repository,
    #[serde(rename = "mistral")]
    Mistral,
}
