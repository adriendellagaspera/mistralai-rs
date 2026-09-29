pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FilterGroup {
    #[serde(rename = "AND")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Box<FilterGroupAndItem>>>,
    #[serde(rename = "OR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Box<FilterGroupOrItem>>>,
}

impl FilterGroup {
    pub fn builder() -> FilterGroupBuilder {
        <FilterGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FilterGroupBuilder {
    and: Option<Vec<Box<FilterGroupAndItem>>>,
    or: Option<Vec<Box<FilterGroupOrItem>>>,
}

impl FilterGroupBuilder {
    pub fn and(mut self, value: Vec<Box<FilterGroupAndItem>>) -> Self {
        self.and = Some(value);
        self
    }

    pub fn or(mut self, value: Vec<Box<FilterGroupOrItem>>) -> Self {
        self.or = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FilterGroup`].
    pub fn build(self) -> Result<FilterGroup, BuildError> {
        Ok(FilterGroup {
            and: self.and,
            or: self.or,
        })
    }
}
