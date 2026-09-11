impl ShareEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Viewer => "Viewer",
            Self::Editor => "Editor",
        }
    }
}
