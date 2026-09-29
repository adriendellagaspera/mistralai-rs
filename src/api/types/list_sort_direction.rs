pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListSortDirection {
    ListSortDirectionUnspecified,
    ListSortDirectionAsc,
    ListSortDirectionDesc,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListSortDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ListSortDirectionUnspecified => {
                serializer.serialize_str("list_sort_direction_unspecified")
            }
            Self::ListSortDirectionAsc => serializer.serialize_str("list_sort_direction_asc"),
            Self::ListSortDirectionDesc => serializer.serialize_str("list_sort_direction_desc"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListSortDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "list_sort_direction_unspecified" => Ok(Self::ListSortDirectionUnspecified),
            "list_sort_direction_asc" => Ok(Self::ListSortDirectionAsc),
            "list_sort_direction_desc" => Ok(Self::ListSortDirectionDesc),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListSortDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ListSortDirectionUnspecified => write!(f, "list_sort_direction_unspecified"),
            Self::ListSortDirectionAsc => write!(f, "list_sort_direction_asc"),
            Self::ListSortDirectionDesc => write!(f, "list_sort_direction_desc"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
