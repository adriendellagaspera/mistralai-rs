///Format of the table
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum OCRTableObjectFormat {
    #[default]
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "html")]
    Html,
}
