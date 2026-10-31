pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponsesFormatTextJsonSchemaConfigType {
    #[serde(rename = "json_schema")]
    JsonSchema,
}
impl fmt::Display for ResponsesFormatTextJsonSchemaConfigType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::JsonSchema => "json_schema",
        };
        write!(f, "{}", s)
    }
}
