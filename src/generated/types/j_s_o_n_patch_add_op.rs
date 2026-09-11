///Add operation
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum JSONPatchAddOp {
    #[default]
    #[serde(rename = "add")]
    Add,
}
