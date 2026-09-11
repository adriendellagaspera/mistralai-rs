#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ToolMessageContent {
    String(String),
    ContentChunkArrayInline78EB73EA0A045B49(ContentChunkArrayInline78EB73EA0A045B49),
}
