impl WandbIntegrationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Wandb => "wandb",
        }
    }
}
