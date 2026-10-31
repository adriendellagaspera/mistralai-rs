pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunProjectSummary {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
}

impl RunProjectSummary {
    pub fn builder() -> RunProjectSummaryBuilder {
        <RunProjectSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunProjectSummaryBuilder {
    id: Option<String>,
    name: Option<String>,
    slug: Option<String>,
}

impl RunProjectSummaryBuilder {
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

    /// Consumes the builder and constructs a [`RunProjectSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RunProjectSummaryBuilder::id)
    /// - [`name`](RunProjectSummaryBuilder::name)
    /// - [`slug`](RunProjectSummaryBuilder::slug)
    pub fn build(self) -> Result<RunProjectSummary, BuildError> {
        Ok(RunProjectSummary {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
