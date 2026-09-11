impl JSONPatchPayloadResponseType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::JsonPatch => "json_patch",
        }
    }
}
