pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLogs {
    #[serde(default)]
    pub logs: FeedResultGetLog,
}

impl GetLogs {
    pub fn builder() -> GetLogsBuilder {
        <GetLogsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLogsBuilder {
    logs: Option<FeedResultGetLog>,
}

impl GetLogsBuilder {
    pub fn logs(mut self, value: FeedResultGetLog) -> Self {
        self.logs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLogs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`logs`](GetLogsBuilder::logs)
    pub fn build(self) -> Result<GetLogs, BuildError> {
        Ok(GetLogs {
            logs: self.logs.ok_or_else(|| BuildError::missing_field("logs"))?,
        })
    }
}
