pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VibeToolCallsByNameStat {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub day: NaiveDate,
    #[serde(default)]
    pub tool_name: String,
}

impl VibeToolCallsByNameStat {
    pub fn builder() -> VibeToolCallsByNameStatBuilder {
        <VibeToolCallsByNameStatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeToolCallsByNameStatBuilder {
    count: Option<i64>,
    day: Option<NaiveDate>,
    tool_name: Option<String>,
}

impl VibeToolCallsByNameStatBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn day(mut self, value: NaiveDate) -> Self {
        self.day = Some(value);
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VibeToolCallsByNameStat`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](VibeToolCallsByNameStatBuilder::count)
    /// - [`day`](VibeToolCallsByNameStatBuilder::day)
    /// - [`tool_name`](VibeToolCallsByNameStatBuilder::tool_name)
    pub fn build(self) -> Result<VibeToolCallsByNameStat, BuildError> {
        Ok(VibeToolCallsByNameStat {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            tool_name: self
                .tool_name
                .ok_or_else(|| BuildError::missing_field("tool_name"))?,
        })
    }
}
