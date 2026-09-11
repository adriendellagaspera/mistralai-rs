#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum AudioContentType {
    #[default]
    #[serde(rename = "audio")]
    Audio,
}
