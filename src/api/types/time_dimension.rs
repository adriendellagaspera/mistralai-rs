pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimeDimension {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<Granularity>,
}

impl TimeDimension {
    pub fn builder() -> TimeDimensionBuilder {
        <TimeDimensionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimeDimensionBuilder {
    granularity: Option<Granularity>,
}

impl TimeDimensionBuilder {
    pub fn granularity(mut self, value: Granularity) -> Self {
        self.granularity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimeDimension`].
    pub fn build(self) -> Result<TimeDimension, BuildError> {
        Ok(TimeDimension {
            granularity: self.granularity,
        })
    }
}
