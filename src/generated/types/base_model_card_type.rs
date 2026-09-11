#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BaseModelCardType {
    #[default]
    #[serde(rename = "base")]
    Base,
}
