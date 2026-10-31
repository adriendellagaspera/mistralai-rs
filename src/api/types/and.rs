pub use crate::prelude::*;

/// Conjunction: rows must match every sub-filter.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct And {
    #[serde(default)]
    pub matches: Vec<Box<AndMatchesItem>>,
}

impl And {
    pub fn builder() -> AndBuilder {
        <AndBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AndBuilder {
    matches: Option<Vec<Box<AndMatchesItem>>>,
}

impl AndBuilder {
    pub fn matches(mut self, value: Vec<Box<AndMatchesItem>>) -> Self {
        self.matches = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`And`].
    /// This method will fail if any of the following fields are not set:
    /// - [`matches`](AndBuilder::matches)
    pub fn build(self) -> Result<And, BuildError> {
        Ok(And {
            matches: self
                .matches
                .ok_or_else(|| BuildError::missing_field("matches"))?,
        })
    }
}
