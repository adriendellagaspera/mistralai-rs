impl EncodingFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Float => "float",
            Self::Base64 => "base64",
        }
    }
}
