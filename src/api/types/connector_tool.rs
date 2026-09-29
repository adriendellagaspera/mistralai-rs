pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_config: Option<ExecutionConfig>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonschema: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<ConnectorToolLocale>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub modified_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    pub visibility: ResourceVisibility,
}

impl ConnectorTool {
    pub fn builder() -> ConnectorToolBuilder {
        <ConnectorToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorToolBuilder {
    active: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    execution_config: Option<ExecutionConfig>,
    id: Option<String>,
    jsonschema: Option<HashMap<String, serde_json::Value>>,
    locale: Option<ConnectorToolLocale>,
    modified_at: Option<DateTime<FixedOffset>>,
    name: Option<String>,
    system_prompt: Option<String>,
    visibility: Option<ResourceVisibility>,
}

impl ConnectorToolBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn execution_config(mut self, value: ExecutionConfig) -> Self {
        self.execution_config = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn jsonschema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.jsonschema = Some(value);
        self
    }

    pub fn locale(mut self, value: ConnectorToolLocale) -> Self {
        self.locale = Some(value);
        self
    }

    pub fn modified_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: ResourceVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ConnectorToolBuilder::created_at)
    /// - [`description`](ConnectorToolBuilder::description)
    /// - [`id`](ConnectorToolBuilder::id)
    /// - [`modified_at`](ConnectorToolBuilder::modified_at)
    /// - [`name`](ConnectorToolBuilder::name)
    /// - [`visibility`](ConnectorToolBuilder::visibility)
    pub fn build(self) -> Result<ConnectorTool, BuildError> {
        Ok(ConnectorTool {
            active: self.active,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            execution_config: self.execution_config,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            jsonschema: self.jsonschema,
            locale: self.locale,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            system_prompt: self.system_prompt,
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
        })
    }
}
