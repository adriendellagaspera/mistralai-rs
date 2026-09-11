#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JSONPatchRemove {
    ///Remove operation
    pub op: JSONPatchRemoveOp,
    ///A JSON Pointer (RFC 6901) identifying the target location within the document. Can be a string path (e.g., '/foo/bar'), '/', '', or an empty list [] for root-level operations.
    pub path: String,
    ///The value to use for the operation
    pub value: serde_json::Value,
}
