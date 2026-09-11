impl ResourceVisibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SharedGlobal => "shared_global",
            Self::SharedOrg => "shared_org",
            Self::SharedWorkspace => "shared_workspace",
            Self::Private => "private",
        }
    }
}
