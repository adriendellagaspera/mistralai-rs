pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromptVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<PromptDefinition>,
    /// Notes for this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Aliases pointing to this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// Creation time.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
}

impl PromptVersion {
    pub fn builder() -> PromptVersionBuilder {
        <PromptVersionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptVersionBuilder {
    version: Option<i64>,
    definition: Option<PromptDefinition>,
    notes: Option<String>,
    aliases: Option<Vec<String>>,
    created_at: Option<Timestamp>,
}

impl PromptVersionBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn definition(mut self, value: PromptDefinition) -> Self {
        self.definition = Some(value);
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

    pub fn created_at(mut self, value: Timestamp) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromptVersion`].
    pub fn build(self) -> Result<PromptVersion, BuildError> {
        Ok(PromptVersion {
            version: self.version,
            definition: self.definition,
            notes: self.notes,
            aliases: self.aliases,
            created_at: self.created_at,
        })
    }
}
