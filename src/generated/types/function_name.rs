///this restriction of `Function` is used to select a specific function to call
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionName {
    pub name: String,
}
