#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostDatasetInSchema {
    ///Constraint: maxLength=200
    pub description: String,
    ///Constraint: minLength=5, maxLength=50
    pub name: String,
}
