impl Serialize for QueryInvocationBodyInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::NetworkEncodedInput(value) => serde::Serialize::serialize(value, serializer),
            Self::QueryInvocationBodyVariant2(value) => {
                serde::Serialize::serialize(value, serializer)
            }
        }
    }
}
