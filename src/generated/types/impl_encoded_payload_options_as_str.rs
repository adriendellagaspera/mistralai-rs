impl EncodedPayloadOptions {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Offloaded => "offloaded",
            Self::Encrypted => "encrypted",
            Self::EncryptedPartial => "encrypted-partial",
        }
    }
}
