pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OptimizationReference {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
}

impl OptimizationReference {
    pub fn builder() -> OptimizationReferenceBuilder {
        <OptimizationReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OptimizationReferenceBuilder {
    id: Option<String>,
    name: Option<String>,
    slug: Option<String>,
}

impl OptimizationReferenceBuilder {
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

    /// Consumes the builder and constructs a [`OptimizationReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OptimizationReferenceBuilder::id)
    /// - [`name`](OptimizationReferenceBuilder::name)
    /// - [`slug`](OptimizationReferenceBuilder::slug)
    pub fn build(self) -> Result<OptimizationReference, BuildError> {
        Ok(OptimizationReference {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
