///Replace operation
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum JSONPatchReplaceOp {
    #[default]
    #[serde(rename = "replace")]
    Replace,
}
