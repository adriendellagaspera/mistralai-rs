impl EntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "User",
            Self::Workspace => "Workspace",
            Self::Org => "Org",
        }
    }
}
