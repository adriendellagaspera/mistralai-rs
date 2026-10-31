pub use crate::prelude::*;

/// Check that the given field falls in a range.
///
/// Requires at least one operator.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Range {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<RangeGt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<RangeGte>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<RangeLt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<RangeLte>,
}

impl Range {
    pub fn builder() -> RangeBuilder {
        <RangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RangeBuilder {
    field: Option<String>,
    gt: Option<RangeGt>,
    gte: Option<RangeGte>,
    lt: Option<RangeLt>,
    lte: Option<RangeLte>,
}

impl RangeBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn gt(mut self, value: RangeGt) -> Self {
        self.gt = Some(value);
        self
    }

    pub fn gte(mut self, value: RangeGte) -> Self {
        self.gte = Some(value);
        self
    }

    pub fn lt(mut self, value: RangeLt) -> Self {
        self.lt = Some(value);
        self
    }

    pub fn lte(mut self, value: RangeLte) -> Self {
        self.lte = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Range`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RangeBuilder::field)
    pub fn build(self) -> Result<Range, BuildError> {
        Ok(Range {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            gt: self.gt,
            gte: self.gte,
            lt: self.lt,
            lte: self.lte,
        })
    }
}
