#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MCPSupportedLanguage {
    #[default]
    #[serde(rename = "en")]
    En,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "nl")]
    Nl,
}
