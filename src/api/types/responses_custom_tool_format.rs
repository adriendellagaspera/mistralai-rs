pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponsesCustomToolFormat {
    ResponsesText(ResponsesText),

    ResponsesGrammar(ResponsesGrammar),
}

impl ResponsesCustomToolFormat {
    pub fn is_responses_text(&self) -> bool {
        matches!(self, Self::ResponsesText(_))
    }

    pub fn is_responses_grammar(&self) -> bool {
        matches!(self, Self::ResponsesGrammar(_))
    }

    pub fn as_responses_text(&self) -> Option<&ResponsesText> {
        match self {
            Self::ResponsesText(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_text(self) -> Option<ResponsesText> {
        match self {
            Self::ResponsesText(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_responses_grammar(&self) -> Option<&ResponsesGrammar> {
        match self {
            Self::ResponsesGrammar(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_grammar(self) -> Option<ResponsesGrammar> {
        match self {
            Self::ResponsesGrammar(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ResponsesCustomToolFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResponsesText(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ResponsesGrammar(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
