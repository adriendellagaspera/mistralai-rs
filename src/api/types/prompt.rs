pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Prompt {
    /// Aliases pointing to this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// Creation time.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<PromptDefinition>,
    /// Display description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Latest version number.
    #[serde(rename = "latestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<i64>,
    /// Stable object name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Notes for this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Registry sharing scope.
    #[serde(rename = "sharingScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_scope: Option<RegistrySharingScope>,
    /// Display title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Last update time.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl Prompt {
    pub fn builder() -> PromptBuilder {
        <PromptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptBuilder {
    aliases: Option<Vec<String>>,
    created_at: Option<Timestamp>,
    definition: Option<PromptDefinition>,
    description: Option<String>,
    id: Option<String>,
    latest_version: Option<i64>,
    name: Option<String>,
    notes: Option<String>,
    sharing_scope: Option<RegistrySharingScope>,
    title: Option<String>,
    updated_at: Option<Timestamp>,
    version: Option<i64>,
}

impl PromptBuilder {
    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn created_at(mut self, value: Timestamp) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn definition(mut self, value: PromptDefinition) -> Self {
        self.definition = Some(value);
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

    pub fn latest_version(mut self, value: i64) -> Self {
        self.latest_version = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn sharing_scope(mut self, value: RegistrySharingScope) -> Self {
        self.sharing_scope = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: Timestamp) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Prompt`].
    pub fn build(self) -> Result<Prompt, BuildError> {
        Ok(Prompt {
            aliases: self.aliases,
            created_at: self.created_at,
            definition: self.definition,
            description: self.description,
            id: self.id,
            latest_version: self.latest_version,
            name: self.name,
            notes: self.notes,
            sharing_scope: self.sharing_scope,
            title: self.title,
            updated_at: self.updated_at,
            version: self.version,
        })
    }
}
