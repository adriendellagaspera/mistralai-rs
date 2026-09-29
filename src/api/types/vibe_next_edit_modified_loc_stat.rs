pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeNextEditModifiedLocStat {
    #[serde(default)]
    pub day: NaiveDate,
    #[serde(default)]
    pub lines_deleted: i64,
    #[serde(default)]
    pub lines_inserted: i64,
    #[serde(default)]
    pub lines_total: i64,
    #[serde(default)]
    pub lines_unchanged: i64,
}

impl VibeNextEditModifiedLocStat {
    pub fn builder() -> VibeNextEditModifiedLocStatBuilder {
        <VibeNextEditModifiedLocStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeNextEditModifiedLocStatBuilder {
    day: Option<NaiveDate>,
    lines_deleted: Option<i64>,
    lines_inserted: Option<i64>,
    lines_total: Option<i64>,
    lines_unchanged: Option<i64>,
}

impl VibeNextEditModifiedLocStatBuilder {
    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
        self
    }

    pub fn lines_deleted(mut self, value: i64) -> Self {
        self.lines_deleted = Some(value);
        self
    }

    pub fn lines_inserted(mut self, value: i64) -> Self {
        self.lines_inserted = Some(value);
        self
    }

    pub fn lines_total(mut self, value: i64) -> Self {
        self.lines_total = Some(value);
        self
    }

    pub fn lines_unchanged(mut self, value: i64) -> Self {
        self.lines_unchanged = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeNextEditModifiedLocStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](VibeNextEditModifiedLocStatBuilder::day)
    /// - [`lines_deleted`](VibeNextEditModifiedLocStatBuilder::lines_deleted)
    /// - [`lines_inserted`](VibeNextEditModifiedLocStatBuilder::lines_inserted)
    /// - [`lines_total`](VibeNextEditModifiedLocStatBuilder::lines_total)
    /// - [`lines_unchanged`](VibeNextEditModifiedLocStatBuilder::lines_unchanged)
    pub fn build(self) -> Result<VibeNextEditModifiedLocStat, BuildError> {
        Ok(VibeNextEditModifiedLocStat {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            lines_deleted: self
                .lines_deleted
                .ok_or_else(|| BuildError::missing_field("lines_deleted"))?,
            lines_inserted: self
                .lines_inserted
                .ok_or_else(|| BuildError::missing_field("lines_inserted"))?,
            lines_total: self
                .lines_total
                .ok_or_else(|| BuildError::missing_field("lines_total"))?,
            lines_unchanged: self
                .lines_unchanged
                .ok_or_else(|| BuildError::missing_field("lines_unchanged"))?,
        })
    }
}
