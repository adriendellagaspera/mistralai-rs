pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunEvaluationSummary {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
}

impl RunEvaluationSummary {
    pub fn builder() -> RunEvaluationSummaryBuilder {
        <RunEvaluationSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunEvaluationSummaryBuilder {
    id: Option<String>,
    name: Option<String>,
    slug: Option<String>,
}

impl RunEvaluationSummaryBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunEvaluationSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RunEvaluationSummaryBuilder::id)
    /// - [`name`](RunEvaluationSummaryBuilder::name)
    /// - [`slug`](RunEvaluationSummaryBuilder::slug)
    pub fn build(self) -> Result<RunEvaluationSummary, BuildError> {
        Ok(RunEvaluationSummary {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
