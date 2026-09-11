#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum GithubRepositoryInType {
    #[default]
    #[serde(rename = "github")]
    Github,
}
