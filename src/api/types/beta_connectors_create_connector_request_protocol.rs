pub use crate::prelude::*;

/// Protocol of the connector. Only 'mcp' is supported on the public endpoint; creating HTTP connectors here is explicitly refused.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateConnectorRequestProtocol {
    #[serde(rename = "mcp")]
    Mcp,
}
impl fmt::Display for CreateConnectorRequestProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Mcp => "mcp",
        };
        write!(f, "{}", s)
    }
}
