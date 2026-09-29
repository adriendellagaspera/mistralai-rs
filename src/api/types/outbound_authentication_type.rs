pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OutboundAuthenticationType {
    Oauth2,
    Bearer,
    None,
    GithubApp,
    SlackApp,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OutboundAuthenticationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Oauth2 => serializer.serialize_str("oauth2"),
            Self::Bearer => serializer.serialize_str("bearer"),
            Self::None => serializer.serialize_str("none"),
            Self::GithubApp => serializer.serialize_str("github_app"),
            Self::SlackApp => serializer.serialize_str("slack_app"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OutboundAuthenticationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "oauth2" => Ok(Self::Oauth2),
            "bearer" => Ok(Self::Bearer),
            "none" => Ok(Self::None),
            "github_app" => Ok(Self::GithubApp),
            "slack_app" => Ok(Self::SlackApp),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OutboundAuthenticationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Oauth2 => write!(f, "oauth2"),
            Self::Bearer => write!(f, "bearer"),
            Self::None => write!(f, "none"),
            Self::GithubApp => write!(f, "github_app"),
            Self::SlackApp => write!(f, "slack_app"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
