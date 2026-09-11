#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImageContentType {
    #[default]
    #[serde(rename = "image")]
    Image,
}
