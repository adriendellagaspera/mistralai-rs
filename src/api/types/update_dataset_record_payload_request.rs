pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateDatasetRecordPayloadRequest {
    #[serde(default)]
    pub payload: DatasetRecordPayload,
}

impl UpdateDatasetRecordPayloadRequest {
    pub fn builder() -> UpdateDatasetRecordPayloadRequestBuilder {
        <UpdateDatasetRecordPayloadRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDatasetRecordPayloadRequestBuilder {
    payload: Option<DatasetRecordPayload>,
}

impl UpdateDatasetRecordPayloadRequestBuilder {
    pub fn payload(mut self, value: DatasetRecordPayload) -> Self {
        self.payload = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDatasetRecordPayloadRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payload`](UpdateDatasetRecordPayloadRequestBuilder::payload)
    pub fn build(self) -> Result<UpdateDatasetRecordPayloadRequest, BuildError> {
        Ok(UpdateDatasetRecordPayloadRequest {
            payload: self
                .payload
                .ok_or_else(|| BuildError::missing_field("payload"))?,
        })
    }
}
