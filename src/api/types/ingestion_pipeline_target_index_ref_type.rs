pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IngestionPipelineTargetIndexRefType {
    #[serde(rename = "vespa")]
    Vespa,
}
impl fmt::Display for IngestionPipelineTargetIndexRefType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Vespa => "vespa",
        };
        write!(f, "{}", s)
    }
}
