#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCREquationBlockType {
    #[default]
    #[serde(rename = "equation")]
    Equation,
}
