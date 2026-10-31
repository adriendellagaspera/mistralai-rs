pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OAuth2AuthorizationCodeAuthMethodMethodType {
    #[serde(rename = "oauth2")]
    Oauth2,
}
impl fmt::Display for OAuth2AuthorizationCodeAuthMethodMethodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Oauth2 => "oauth2",
        };
        write!(f, "{}", s)
    }
}
