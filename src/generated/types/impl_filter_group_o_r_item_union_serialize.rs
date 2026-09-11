impl Serialize for FilterGroupORItemUnion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::FilterGroup(value) => serde::Serialize::serialize(value, serializer),
            Self::FilterCondition(value) => serde::Serialize::serialize(value, serializer),
        }
    }
}
