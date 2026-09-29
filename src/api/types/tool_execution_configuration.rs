pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolExecutionConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_confirmation: Option<ToolExecutionConfigurationRequiresConfirmation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_confirmation: Option<ToolExecutionConfigurationSkipConfirmation>,
}

impl ToolExecutionConfiguration {
    pub fn builder() -> ToolExecutionConfigurationBuilder {
        <ToolExecutionConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolExecutionConfigurationBuilder {
    exclude: Option<Vec<String>>,
    include: Option<Vec<String>>,
    requires_confirmation: Option<ToolExecutionConfigurationRequiresConfirmation>,
    skip_confirmation: Option<ToolExecutionConfigurationSkipConfirmation>,
}

impl ToolExecutionConfigurationBuilder {
    pub fn exclude(mut self, value: Vec<String>) -> Self {
        self.exclude = Some(value);
        self
    }

    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }

    pub fn requires_confirmation(
        mut self,
        value: ToolExecutionConfigurationRequiresConfirmation,
    ) -> Self {
        self.requires_confirmation = Some(value);
        self
    }

    pub fn skip_confirmation(mut self, value: ToolExecutionConfigurationSkipConfirmation) -> Self {
        self.skip_confirmation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolExecutionConfiguration`].
    pub fn build(self) -> Result<ToolExecutionConfiguration, BuildError> {
        Ok(ToolExecutionConfiguration {
            exclude: self.exclude,
            include: self.include,
            requires_confirmation: self.requires_confirmation,
            skip_confirmation: self.skip_confirmation,
        })
    }
}
