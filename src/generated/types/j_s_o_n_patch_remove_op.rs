///Remove operation
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum JSONPatchRemoveOp {
    #[default]
    #[serde(rename = "remove")]
    Remove,
}
