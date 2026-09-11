#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstructRequest {
    pub messages: Vec<InstructRequestMessagesItemUnion>,
}
