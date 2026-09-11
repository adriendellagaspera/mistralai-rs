impl MCPSupportedLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Fr => "fr",
            Self::De => "de",
            Self::Es => "es",
            Self::Pl => "pl",
            Self::It => "it",
            Self::Ar => "ar",
            Self::PtBr => "pt-BR",
            Self::Nl => "nl",
        }
    }
}
