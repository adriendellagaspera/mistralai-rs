pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SkillVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<SkillDefinition>,
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

impl SkillVersion {
    pub fn builder() -> SkillVersionBuilder {
        <SkillVersionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkillVersionBuilder {
    version: Option<i64>,
    definition: Option<SkillDefinition>,
    notes: Option<String>,
    aliases: Option<Vec<String>>,
    created_at: Option<Timestamp>,
}

impl SkillVersionBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn definition(mut self, value: SkillDefinition) -> Self {
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

    /// Consumes the builder and constructs a [`SkillVersion`].
    pub fn build(self) -> Result<SkillVersion, BuildError> {
        Ok(SkillVersion {
            version: self.version,
            definition: self.definition,
            notes: self.notes,
            aliases: self.aliases,
            created_at: self.created_at,
        })
    }
}
