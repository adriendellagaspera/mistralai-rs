pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FilterGroupAndItem {
    FilterGroup(Box<FilterGroup>),

    FilterCondition(FilterCondition),
}

impl FilterGroupAndItem {
    pub fn is_filter_group(&self) -> bool {
        matches!(self, Self::FilterGroup(_))
    }

    pub fn is_filter_condition(&self) -> bool {
        matches!(self, Self::FilterCondition(_))
    }

    pub fn as_filter_group(&self) -> Option<&Box<FilterGroup>> {
        match self {
            Self::FilterGroup(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_filter_group(self) -> Option<FilterGroup> {
        match self {
            Self::FilterGroup(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_filter_condition(&self) -> Option<&FilterCondition> {
        match self {
            Self::FilterCondition(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_filter_condition(self) -> Option<FilterCondition> {
        match self {
            Self::FilterCondition(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for FilterGroupAndItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FilterGroup(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::FilterCondition(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
