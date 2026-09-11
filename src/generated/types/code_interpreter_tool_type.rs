#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CodeInterpreterToolType {
    #[default]
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
}
