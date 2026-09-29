pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeUserPromptsStat {
    #[serde(default)]
    pub day: NaiveDate,
    #[serde(default)]
    pub nb_prompts_acp: i64,
    #[serde(default)]
    pub nb_prompts_cli: i64,
    #[serde(default)]
    pub nb_prompts_programmatic: i64,
    #[serde(default)]
    pub nb_prompts_total: i64,
}

impl VibeUserPromptsStat {
    pub fn builder() -> VibeUserPromptsStatBuilder {
        <VibeUserPromptsStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeUserPromptsStatBuilder {
    day: Option<NaiveDate>,
    nb_prompts_acp: Option<i64>,
    nb_prompts_cli: Option<i64>,
    nb_prompts_programmatic: Option<i64>,
    nb_prompts_total: Option<i64>,
}

impl VibeUserPromptsStatBuilder {
    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
        self
    }

    pub fn nb_prompts_acp(mut self, value: i64) -> Self {
        self.nb_prompts_acp = Some(value);
        self
    }

    pub fn nb_prompts_cli(mut self, value: i64) -> Self {
        self.nb_prompts_cli = Some(value);
        self
    }

    pub fn nb_prompts_programmatic(mut self, value: i64) -> Self {
        self.nb_prompts_programmatic = Some(value);
        self
    }

    pub fn nb_prompts_total(mut self, value: i64) -> Self {
        self.nb_prompts_total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeUserPromptsStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](VibeUserPromptsStatBuilder::day)
    /// - [`nb_prompts_acp`](VibeUserPromptsStatBuilder::nb_prompts_acp)
    /// - [`nb_prompts_cli`](VibeUserPromptsStatBuilder::nb_prompts_cli)
    /// - [`nb_prompts_programmatic`](VibeUserPromptsStatBuilder::nb_prompts_programmatic)
    /// - [`nb_prompts_total`](VibeUserPromptsStatBuilder::nb_prompts_total)
    pub fn build(self) -> Result<VibeUserPromptsStat, BuildError> {
        Ok(VibeUserPromptsStat {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            nb_prompts_acp: self
                .nb_prompts_acp
                .ok_or_else(|| BuildError::missing_field("nb_prompts_acp"))?,
            nb_prompts_cli: self
                .nb_prompts_cli
                .ok_or_else(|| BuildError::missing_field("nb_prompts_cli"))?,
            nb_prompts_programmatic: self
                .nb_prompts_programmatic
                .ok_or_else(|| BuildError::missing_field("nb_prompts_programmatic"))?,
            nb_prompts_total: self
                .nb_prompts_total
                .ok_or_else(|| BuildError::missing_field("nb_prompts_total"))?,
        })
    }
}
