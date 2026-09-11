///Chat to classify
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ChatModerationRequestInput {
    ChatModerationRequestInputArray(ChatModerationRequestInputArray),
    ChatModerationRequestInputArrayInline(ChatModerationRequestInputArrayInline),
}
