impl ResponseFormats {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::JsonObject => "json_object",
            Self::JsonSchema => "json_schema",
        }
    }
}
