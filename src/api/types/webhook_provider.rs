pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookProvider {
    Slack,
    Github,
    Notion,
    Lucca,
    Linear,
    Microsoft,
    Ashby,
    Whatsapp,
    Custom,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookProvider {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Slack => serializer.serialize_str("slack"),
            Self::Github => serializer.serialize_str("github"),
            Self::Notion => serializer.serialize_str("notion"),
            Self::Lucca => serializer.serialize_str("lucca"),
            Self::Linear => serializer.serialize_str("linear"),
            Self::Microsoft => serializer.serialize_str("microsoft"),
            Self::Ashby => serializer.serialize_str("ashby"),
            Self::Whatsapp => serializer.serialize_str("whatsapp"),
            Self::Custom => serializer.serialize_str("custom"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "slack" => Ok(Self::Slack),
            "github" => Ok(Self::Github),
            "notion" => Ok(Self::Notion),
            "lucca" => Ok(Self::Lucca),
            "linear" => Ok(Self::Linear),
            "microsoft" => Ok(Self::Microsoft),
            "ashby" => Ok(Self::Ashby),
            "whatsapp" => Ok(Self::Whatsapp),
            "custom" => Ok(Self::Custom),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Slack => write!(f, "slack"),
            Self::Github => write!(f, "github"),
            Self::Notion => write!(f, "notion"),
            Self::Lucca => write!(f, "lucca"),
            Self::Linear => write!(f, "linear"),
            Self::Microsoft => write!(f, "microsoft"),
            Self::Ashby => write!(f, "ashby"),
            Self::Whatsapp => write!(f, "whatsapp"),
            Self::Custom => write!(f, "custom"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
