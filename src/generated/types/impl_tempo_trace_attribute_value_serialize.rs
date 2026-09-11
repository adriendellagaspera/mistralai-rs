impl Serialize for TempoTraceAttributeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::TempoTraceAttributeStringValue(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::TempoTraceAttributeIntValue(value) => {
                serde::Serialize::serialize(value, serializer)
            }
            Self::TempoTraceAttributeBoolValue(value) => {
                serde::Serialize::serialize(value, serializer)
            }
        }
    }
}
