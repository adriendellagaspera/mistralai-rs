#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct FilterGroup {
    #[serde(
        rename = "AND",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub and: Option<Option<Vec<Box<FilterGroupANDItemUnion>>>>,
    #[serde(
        rename = "OR",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub or: Option<Option<Vec<Box<FilterGroupORItemUnion>>>>,
}
