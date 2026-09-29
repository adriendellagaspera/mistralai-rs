pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectorTool {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<ConnectorToolLocale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonschema: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_config: Option<ExecutionConfig>,
    pub visibility: ResourceVisibility,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub modified_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl ConnectorTool {
    pub fn builder() -> ConnectorToolBuilder {
        <ConnectorToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorToolBuilder {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    system_prompt: Option<String>,
    locale: Option<ConnectorToolLocale>,
    jsonschema: Option<HashMap<String, serde_json::Value>>,
    execution_config: Option<ExecutionConfig>,
    visibility: Option<ResourceVisibility>,
    created_at: Option<DateTime<FixedOffset>>,
    modified_at: Option<DateTime<FixedOffset>>,
    active: Option<bool>,
}

impl ConnectorToolBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    pub fn locale(mut self, value: ConnectorToolLocale) -> Self {
        self.locale = Some(value);
        self
    }

    pub fn jsonschema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.jsonschema = Some(value);
        self
    }

    pub fn execution_config(mut self, value: ExecutionConfig) -> Self {
        self.execution_config = Some(value);
        self
    }

    pub fn visibility(mut self, value: ResourceVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn modified_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConnectorToolBuilder::id)
    /// - [`name`](ConnectorToolBuilder::name)
    /// - [`description`](ConnectorToolBuilder::description)
    /// - [`visibility`](ConnectorToolBuilder::visibility)
    /// - [`created_at`](ConnectorToolBuilder::created_at)
    /// - [`modified_at`](ConnectorToolBuilder::modified_at)
    pub fn build(self) -> Result<ConnectorTool, BuildError> {
        Ok(ConnectorTool {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            system_prompt: self.system_prompt,
            locale: self.locale,
            jsonschema: self.jsonschema,
            execution_config: self.execution_config,
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            active: self.active,
        })
    }
}
