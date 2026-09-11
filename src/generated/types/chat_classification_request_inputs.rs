///Chat to classify
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ChatClassificationRequestInputs {
    InstructRequest(InstructRequest),
    InstructRequestArray(InstructRequestArray),
}
