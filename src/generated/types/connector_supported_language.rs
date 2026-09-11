#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ConnectorSupportedLanguage {
    #[default]
    #[serde(rename = "en")]
    En,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "nl")]
    Nl,
}
