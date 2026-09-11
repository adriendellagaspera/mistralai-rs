#[derive(Debug, Clone)]
pub enum JSONPatchPayloadResponseValueItemUnion {
    JSONPatchAppend(JSONPatchAppend),
    JSONPatchAdd(JSONPatchAdd),
    JSONPatchReplace(JSONPatchReplace),
    JSONPatchRemove(JSONPatchRemove),
}
