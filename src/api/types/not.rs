pub use crate::prelude::*;

/// Negate a filter: rows must not match.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Not {
    pub r#match: Box<NotMatch>,
}

impl Not {
    pub fn builder() -> NotBuilder {
        <NotBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotBuilder {
    r#match: Option<Box<NotMatch>>,
}

impl NotBuilder {
    pub fn r#match(mut self, value: Box<NotMatch>) -> Self {
        self.r#match = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Not`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#match`](NotBuilder::r#match)
    pub fn build(self) -> Result<Not, BuildError> {
        Ok(Not {
            r#match: self
                .r#match
                .ok_or_else(|| BuildError::missing_field("r#match"))?,
        })
    }
}
