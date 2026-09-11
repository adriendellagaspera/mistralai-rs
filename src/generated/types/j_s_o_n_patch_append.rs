#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JSONPatchAppend {
    ///'append' is an extension for efficient string concatenation in streaming scenarios.
    pub op: JSONPatchAppendOp,
    ///A JSON Pointer (RFC 6901) identifying the target location within the document. Can be a string path (e.g., '/foo/bar'), '/', '', or an empty list [] for root-level operations.
    pub path: String,
    ///The value to use for the operation. A string to append to the existing value
    pub value: String,
}
