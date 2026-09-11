impl ConnectorSupportedLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Fr => "fr",
            Self::Ar => "ar",
            Self::Es => "es",
            Self::De => "de",
            Self::Pl => "pl",
            Self::PtBr => "pt-BR",
            Self::It => "it",
            Self::Nl => "nl",
        }
    }
}
