pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OAuth2TokenTokenType {
    Bearer,
}
impl fmt::Display for OAuth2TokenTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Bearer => "Bearer",
        };
        write!(f, "{}", s)
    }
}
