///'append' is an extension for efficient string concatenation in streaming scenarios.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum JSONPatchAppendOp {
    #[default]
    #[serde(rename = "append")]
    Append,
}
