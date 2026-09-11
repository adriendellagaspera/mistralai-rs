impl GithubRepositoryOutType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Github => "github",
        }
    }
}
