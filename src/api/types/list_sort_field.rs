pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListSortField {
    ListSortFieldUnspecified,
    ListSortFieldCreatedAt,
    ListSortFieldLastModifiedAt,
    ListSortFieldName,
    ListSortFieldTitle,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListSortField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ListSortFieldUnspecified => {
                serializer.serialize_str("list_sort_field_unspecified")
            }
            Self::ListSortFieldCreatedAt => serializer.serialize_str("list_sort_field_created_at"),
            Self::ListSortFieldLastModifiedAt => {
                serializer.serialize_str("list_sort_field_last_modified_at")
            }
            Self::ListSortFieldName => serializer.serialize_str("list_sort_field_name"),
            Self::ListSortFieldTitle => serializer.serialize_str("list_sort_field_title"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListSortField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "list_sort_field_unspecified" => Ok(Self::ListSortFieldUnspecified),
            "list_sort_field_created_at" => Ok(Self::ListSortFieldCreatedAt),
            "list_sort_field_last_modified_at" => Ok(Self::ListSortFieldLastModifiedAt),
            "list_sort_field_name" => Ok(Self::ListSortFieldName),
            "list_sort_field_title" => Ok(Self::ListSortFieldTitle),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ListSortFieldUnspecified => write!(f, "list_sort_field_unspecified"),
            Self::ListSortFieldCreatedAt => write!(f, "list_sort_field_created_at"),
            Self::ListSortFieldLastModifiedAt => write!(f, "list_sort_field_last_modified_at"),
            Self::ListSortFieldName => write!(f, "list_sort_field_name"),
            Self::ListSortFieldTitle => write!(f, "list_sort_field_title"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
