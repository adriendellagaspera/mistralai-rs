pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SkillVersion {
    /// Aliases pointing to this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// Creation time.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<SkillDefinition>,
    /// Notes for this version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl SkillVersion {
    pub fn builder() -> SkillVersionBuilder {
        <SkillVersionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkillVersionBuilder {
    aliases: Option<Vec<String>>,
    created_at: Option<Timestamp>,
    definition: Option<SkillDefinition>,
    notes: Option<String>,
    version: Option<i64>,
}

impl SkillVersionBuilder {
    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn created_at(mut self, value: Timestamp) -> Self {
        self.created_at = Some(value);
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

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SkillVersion`].
    pub fn build(self) -> Result<SkillVersion, BuildError> {
        Ok(SkillVersion {
            aliases: self.aliases,
            created_at: self.created_at,
            definition: self.definition,
            notes: self.notes,
            version: self.version,
        })
    }
}
