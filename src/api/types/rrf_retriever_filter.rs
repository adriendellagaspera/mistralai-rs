pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum RrfRetrieverFilter {
    #[serde(rename = "and")]
    #[non_exhaustive]
    And {
        #[serde(flatten)]
        data: And,
    },

    #[serde(rename = "equal")]
    #[non_exhaustive]
    Equal {
        #[serde(flatten)]
        data: Equal,
    },

    #[serde(rename = "in")]
    #[non_exhaustive]
    In {
        #[serde(flatten)]
        data: In,
    },

    #[serde(rename = "not")]
    #[non_exhaustive]
    Not {
        #[serde(flatten)]
        data: Not,
    },

    #[serde(rename = "or")]
    #[non_exhaustive]
    Or {
        #[serde(flatten)]
        data: Or,
    },

    #[serde(rename = "range")]
    #[non_exhaustive]
    Range {
        #[serde(flatten)]
        data: Range,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl RrfRetrieverFilter {
    pub fn and(data: And) -> Self {
        Self::And { data }
    }

    pub fn equal(data: Equal) -> Self {
        Self::Equal { data }
    }

    pub fn r#in(data: In) -> Self {
        Self::In { data }
    }

    pub fn not(data: Not) -> Self {
        Self::Not { data }
    }

    pub fn or(data: Or) -> Self {
        Self::Or { data }
    }

    pub fn range(data: Range) -> Self {
        Self::Range { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
