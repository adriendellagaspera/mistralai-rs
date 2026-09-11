#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum WandbIntegrationType {
    #[default]
    #[serde(rename = "wandb")]
    Wandb,
}
