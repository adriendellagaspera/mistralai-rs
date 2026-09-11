#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum WandbIntegrationOutType {
    #[default]
    #[serde(rename = "wandb")]
    Wandb,
}
