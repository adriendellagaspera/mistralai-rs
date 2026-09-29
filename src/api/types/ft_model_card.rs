pub use crate::prelude::*;

/// Extra fields for fine-tuned models.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FtModelCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default)]
    pub capabilities: ModelCapabilities,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_replacement_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(default)]
    pub job: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_context_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_by: Option<String>,
    #[serde(default)]
    pub root: String,
}

impl FtModelCard {
    pub fn builder() -> FtModelCardBuilder {
        <FtModelCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FtModelCardBuilder {
    aliases: Option<Vec<String>>,
    archived: Option<bool>,
    capabilities: Option<ModelCapabilities>,
    created: Option<i64>,
    default_model_temperature: Option<f64>,
    deprecation: Option<DateTime<FixedOffset>>,
    deprecation_replacement_model: Option<String>,
    description: Option<String>,
    id: Option<String>,
    internal: Option<bool>,
    job: Option<String>,
    max_context_length: Option<i64>,
    name: Option<String>,
    object: Option<String>,
    owned_by: Option<String>,
    root: Option<String>,
}

impl FtModelCardBuilder {
    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn capabilities(mut self, value: ModelCapabilities) -> Self {
        self.capabilities = Some(value);
        self
    }

    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn default_model_temperature(mut self, value: f64) -> Self {
        self.default_model_temperature = Some(value);
        self
    }

    pub fn deprecation(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deprecation = Some(value);
        self
    }

    pub fn deprecation_replacement_model(mut self, value: impl Into<String>) -> Self {
        self.deprecation_replacement_model = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn internal(mut self, value: bool) -> Self {
        self.internal = Some(value);
        self
    }

    pub fn job(mut self, value: impl Into<String>) -> Self {
        self.job = Some(value.into());
        self
    }

    pub fn max_context_length(mut self, value: i64) -> Self {
        self.max_context_length = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn owned_by(mut self, value: impl Into<String>) -> Self {
        self.owned_by = Some(value.into());
        self
    }

    pub fn root(mut self, value: impl Into<String>) -> Self {
        self.root = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FtModelCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capabilities`](FtModelCardBuilder::capabilities)
    /// - [`id`](FtModelCardBuilder::id)
    /// - [`job`](FtModelCardBuilder::job)
    /// - [`root`](FtModelCardBuilder::root)
    pub fn build(self) -> Result<FtModelCard, BuildError> {
        Ok(FtModelCard {
            aliases: self.aliases,
            archived: self.archived,
            capabilities: self
                .capabilities
                .ok_or_else(|| BuildError::missing_field("capabilities"))?,
            created: self.created,
            default_model_temperature: self.default_model_temperature,
            deprecation: self.deprecation,
            deprecation_replacement_model: self.deprecation_replacement_model,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            internal: self.internal,
            job: self.job.ok_or_else(|| BuildError::missing_field("job"))?,
            max_context_length: self.max_context_length,
            name: self.name,
            object: self.object,
            owned_by: self.owned_by,
            root: self.root.ok_or_else(|| BuildError::missing_field("root"))?,
        })
    }
}
