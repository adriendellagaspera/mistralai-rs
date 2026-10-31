pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponsesSummaryType {
    #[serde(rename = "summary_text")]
    SummaryText,
}
impl fmt::Display for ResponsesSummaryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SummaryText => "summary_text",
        };
        write!(f, "{}", s)
    }
}
