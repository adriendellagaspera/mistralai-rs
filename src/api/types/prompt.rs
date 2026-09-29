pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Prompt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Stable object name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<PromptDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    /// Notes for this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Aliases pointing to this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// Registry sharing scope.
    #[serde(rename = "sharingScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_scope: Option<RegistrySharingScope>,
    /// Creation time.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// Last update time.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
    /// Latest version number.
    #[serde(rename = "latestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<i64>,
    /// Display title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Display description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Prompt {
    pub fn builder() -> PromptBuilder {
        <PromptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptBuilder {
    id: Option<String>,
    name: Option<String>,
    definition: Option<PromptDefinition>,
    version: Option<i64>,
    notes: Option<String>,
    aliases: Option<Vec<String>>,
    sharing_scope: Option<RegistrySharingScope>,
    created_at: Option<Timestamp>,
    updated_at: Option<Timestamp>,
    latest_version: Option<i64>,
    title: Option<String>,
    description: Option<String>,
}

impl PromptBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn definition(mut self, value: PromptDefinition) -> Self {
        self.definition = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn sharing_scope(mut self, value: RegistrySharingScope) -> Self {
        self.sharing_scope = Some(value);
        self
    }

    pub fn created_at(mut self, value: Timestamp) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: Timestamp) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn latest_version(mut self, value: i64) -> Self {
        self.latest_version = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Prompt`].
    pub fn build(self) -> Result<Prompt, BuildError> {
        Ok(Prompt {
            id: self.id,
            name: self.name,
            definition: self.definition,
            version: self.version,
            notes: self.notes,
            aliases: self.aliases,
            sharing_scope: self.sharing_scope,
            created_at: self.created_at,
            updated_at: self.updated_at,
            latest_version: self.latest_version,
            title: self.title,
            description: self.description,
        })
    }
}
