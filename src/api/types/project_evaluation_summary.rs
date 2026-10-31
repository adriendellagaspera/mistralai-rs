pub use crate::prelude::*;

/// A lightweight evaluation reference embedded in the projects listing, so the client can render a
/// project's evaluations without firing one request per project.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProjectEvaluationSummary {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
}

impl ProjectEvaluationSummary {
    pub fn builder() -> ProjectEvaluationSummaryBuilder {
        <ProjectEvaluationSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectEvaluationSummaryBuilder {
    id: Option<String>,
    name: Option<String>,
    slug: Option<String>,
}

impl ProjectEvaluationSummaryBuilder {
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

    /// Consumes the builder and constructs a [`ProjectEvaluationSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProjectEvaluationSummaryBuilder::id)
    /// - [`name`](ProjectEvaluationSummaryBuilder::name)
    /// - [`slug`](ProjectEvaluationSummaryBuilder::slug)
    pub fn build(self) -> Result<ProjectEvaluationSummary, BuildError> {
        Ok(ProjectEvaluationSummary {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
