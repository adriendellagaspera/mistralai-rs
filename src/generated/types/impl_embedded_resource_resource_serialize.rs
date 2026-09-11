impl Serialize for EmbeddedResourceResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::TextResourceContents(value) => serde::Serialize::serialize(value, serializer),
            Self::BlobResourceContents(value) => serde::Serialize::serialize(value, serializer),
        }
    }
}
