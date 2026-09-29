pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_confirmation: Option<Vec<String>>,
}

impl ToolConfiguration {
    pub fn builder() -> ToolConfigurationBuilder {
        <ToolConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolConfigurationBuilder {
    exclude: Option<Vec<String>>,
    include: Option<Vec<String>>,
    requires_confirmation: Option<Vec<String>>,
}

impl ToolConfigurationBuilder {
    pub fn exclude(mut self, value: Vec<String>) -> Self {
        self.exclude = Some(value);
        self
    }

    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }

    pub fn requires_confirmation(mut self, value: Vec<String>) -> Self {
        self.requires_confirmation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolConfiguration`].
    pub fn build(self) -> Result<ToolConfiguration, BuildError> {
        Ok(ToolConfiguration {
            exclude: self.exclude,
            include: self.include,
            requires_confirmation: self.requires_confirmation,
        })
    }
}
