pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateRunInfo {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub execution_time: DateTime<FixedOffset>,
    #[serde(default)]
    pub chunks_count: i64,
}

impl UpdateRunInfo {
    pub fn builder() -> UpdateRunInfoBuilder {
        <UpdateRunInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateRunInfoBuilder {
    execution_time: Option<DateTime<FixedOffset>>,
    chunks_count: Option<i64>,
}

impl UpdateRunInfoBuilder {
    pub fn execution_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.execution_time = Some(value);
        self
    }

    pub fn chunks_count(mut self, value: i64) -> Self {
        self.chunks_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateRunInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`execution_time`](UpdateRunInfoBuilder::execution_time)
    /// - [`chunks_count`](UpdateRunInfoBuilder::chunks_count)
    pub fn build(self) -> Result<UpdateRunInfo, BuildError> {
        Ok(UpdateRunInfo {
            execution_time: self
                .execution_time
                .ok_or_else(|| BuildError::missing_field("execution_time"))?,
            chunks_count: self
                .chunks_count
                .ok_or_else(|| BuildError::missing_field("chunks_count"))?,
        })
    }
}
