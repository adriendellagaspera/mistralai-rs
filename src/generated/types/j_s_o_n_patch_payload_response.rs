/**A payload containing a list of JSON Patch operations.

Used for streaming incremental updates to workflow state.*/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JSONPatchPayloadResponse {
    ///Discriminator indicating this is a JSON Patch payload.
    #[serde(default)]
    pub r#type: JSONPatchPayloadResponseType,
    ///The list of JSON Patch operations to apply in order.
    pub value: Vec<JSONPatchPayloadResponseValueItemUnion>,
}
