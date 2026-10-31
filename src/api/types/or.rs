pub use crate::prelude::*;

/// Disjunction: rows must match at least one sub-filter.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Or {
    #[serde(default)]
    pub matches: Vec<Box<OrMatchesItem>>,
}

impl Or {
    pub fn builder() -> OrBuilder {
        <OrBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrBuilder {
    matches: Option<Vec<Box<OrMatchesItem>>>,
}

impl OrBuilder {
    pub fn matches(mut self, value: Vec<Box<OrMatchesItem>>) -> Self {
        self.matches = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Or`].
    /// This method will fail if any of the following fields are not set:
    /// - [`matches`](OrBuilder::matches)
    pub fn build(self) -> Result<Or, BuildError> {
        Ok(Or {
            matches: self
                .matches
                .ok_or_else(|| BuildError::missing_field("matches"))?,
        })
    }
}
