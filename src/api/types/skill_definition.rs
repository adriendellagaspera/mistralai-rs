pub use crate::prelude::*;

/// Versioned skill content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SkillDefinition {
    /// Model-facing trigger and usage description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Skill body content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Additional files available to the skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<HashMap<String, SkillAssetContent>>,
}

impl SkillDefinition {
    pub fn builder() -> SkillDefinitionBuilder {
        <SkillDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkillDefinitionBuilder {
    description: Option<String>,
    body: Option<String>,
    assets: Option<HashMap<String, SkillAssetContent>>,
}

impl SkillDefinitionBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn assets(mut self, value: HashMap<String, SkillAssetContent>) -> Self {
        self.assets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SkillDefinition`].
    pub fn build(self) -> Result<SkillDefinition, BuildError> {
        Ok(SkillDefinition {
            description: self.description,
            body: self.body,
            assets: self.assets,
        })
    }
}
