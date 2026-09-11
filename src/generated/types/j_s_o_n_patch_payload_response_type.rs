///Discriminator indicating this is a JSON Patch payload.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum JSONPatchPayloadResponseType {
    #[default]
    #[serde(rename = "json_patch")]
    JsonPatch,
}
