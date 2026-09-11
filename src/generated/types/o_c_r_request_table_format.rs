#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRRequestTableFormat {
    #[default]
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "html")]
    Html,
}
