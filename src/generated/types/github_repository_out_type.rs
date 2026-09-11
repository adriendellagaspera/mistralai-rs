#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum GithubRepositoryOutType {
    #[default]
    #[serde(rename = "github")]
    Github,
}
