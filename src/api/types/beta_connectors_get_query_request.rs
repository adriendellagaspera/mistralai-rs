pub use crate::prelude::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaConnectorsGetQueryRequest {
    /// Fetch the user-level data associated with the connector (e.g. connection credentials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_user_data: Option<bool>,
}

impl BetaConnectorsGetQueryRequest {
    pub fn builder() -> BetaConnectorsGetQueryRequestBuilder {
        <BetaConnectorsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaConnectorsGetQueryRequestBuilder {
    fetch_user_data: Option<bool>,
}

impl BetaConnectorsGetQueryRequestBuilder {
    pub fn fetch_user_data(mut self, value: bool) -> Self {
        self.fetch_user_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BetaConnectorsGetQueryRequest`].
    pub fn build(self) -> Result<BetaConnectorsGetQueryRequest, BuildError> {
        Ok(BetaConnectorsGetQueryRequest {
            fetch_user_data: self.fetch_user_data,
        })
    }
}
