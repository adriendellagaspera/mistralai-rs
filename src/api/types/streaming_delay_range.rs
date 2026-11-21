pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StreamingDelayRange {
    #[serde(default)]
    pub max: i64,
    #[serde(default)]
    pub min: i64,
}

impl StreamingDelayRange {
    pub fn builder() -> StreamingDelayRangeBuilder {
        <StreamingDelayRangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamingDelayRangeBuilder {
    max: Option<i64>,
    min: Option<i64>,
}

impl StreamingDelayRangeBuilder {
    pub fn max(mut self, value: i64) -> Self {
        self.max = Some(value);
        self
    }

    pub fn min(mut self, value: i64) -> Self {
        self.min = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StreamingDelayRange`].
    /// This method will fail if any of the following fields are not set:
    /// - [`max`](StreamingDelayRangeBuilder::max)
    /// - [`min`](StreamingDelayRangeBuilder::min)
    pub fn build(self) -> Result<StreamingDelayRange, BuildError> {
        Ok(StreamingDelayRange {
            max: self.max.ok_or_else(|| BuildError::missing_field("max"))?,
            min: self.min.ok_or_else(|| BuildError::missing_field("min"))?,
        })
    }
}
