pub use crate::prelude::*;

/// Versioned skill content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SkillDefinition {
    /// Additional files available to the skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<HashMap<String, SkillAssetContent>>,
    /// Skill body content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Model-facing trigger and usage description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl SkillDefinition {
    pub fn builder() -> SkillDefinitionBuilder {
        <SkillDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkillDefinitionBuilder {
    assets: Option<HashMap<String, SkillAssetContent>>,
    body: Option<String>,
    description: Option<String>,
}

impl SkillDefinitionBuilder {
    pub fn assets(mut self, value: HashMap<String, SkillAssetContent>) -> Self {
        self.assets = Some(value);
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SkillDefinition`].
    pub fn build(self) -> Result<SkillDefinition, BuildError> {
        Ok(SkillDefinition {
            assets: self.assets,
            body: self.body,
            description: self.description,
        })
    }
}
