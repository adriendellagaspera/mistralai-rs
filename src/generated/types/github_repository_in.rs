#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GithubRepositoryIn {
    pub name: String,
    pub owner: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub r#ref: Option<Option<String>>,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<GithubRepositoryInType>,
    ///Constraint: exclusiveMinimum=0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}
