pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TagFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combinator: Option<FilterCombinator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl TagFilter {
    pub fn builder() -> TagFilterBuilder {
        <TagFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TagFilterBuilder {
    combinator: Option<FilterCombinator>,
    values: Option<Vec<String>>,
}

impl TagFilterBuilder {
    pub fn combinator(mut self, value: FilterCombinator) -> Self {
        self.combinator = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<String>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TagFilter`].
    pub fn build(self) -> Result<TagFilter, BuildError> {
        Ok(TagFilter {
            combinator: self.combinator,
            values: self.values,
        })
    }
}
