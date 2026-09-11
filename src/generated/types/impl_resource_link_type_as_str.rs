impl ResourceLinkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ResourceLink => "resource_link",
        }
    }
}
