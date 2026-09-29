pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModerationLlmv2Config {
    /// Override model name. Should be omitted in general.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_category_thresholds: Option<ModerationLlmv2CategoryThresholds>,
    /// If true, only evaluate categories in custom_category_thresholds; others are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_other_categories: Option<bool>,
    /// Action to take if any score is above the threshold for any category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ModerationLlmAction>,
}

impl ModerationLlmv2Config {
    pub fn builder() -> ModerationLlmv2ConfigBuilder {
        <ModerationLlmv2ConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModerationLlmv2ConfigBuilder {
    model_name: Option<String>,
    custom_category_thresholds: Option<ModerationLlmv2CategoryThresholds>,
    ignore_other_categories: Option<bool>,
    action: Option<ModerationLlmAction>,
}

impl ModerationLlmv2ConfigBuilder {
    pub fn model_name(mut self, value: impl Into<String>) -> Self {
        self.model_name = Some(value.into());
        self
    }

    pub fn custom_category_thresholds(mut self, value: ModerationLlmv2CategoryThresholds) -> Self {
        self.custom_category_thresholds = Some(value);
        self
    }

    pub fn ignore_other_categories(mut self, value: bool) -> Self {
        self.ignore_other_categories = Some(value);
        self
    }

    pub fn action(mut self, value: ModerationLlmAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModerationLlmv2Config`].
    pub fn build(self) -> Result<ModerationLlmv2Config, BuildError> {
        Ok(ModerationLlmv2Config {
            model_name: self.model_name,
            custom_category_thresholds: self.custom_category_thresholds,
            ignore_other_categories: self.ignore_other_categories,
            action: self.action,
        })
    }
}
