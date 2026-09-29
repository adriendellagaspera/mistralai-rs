pub use crate::prelude::*;

/// Query parameters for connector_get_v1
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectorGetV1QueryRequest {
    /// Fetch the user-level data associated with the connector (e.g. connection credentials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_user_data: Option<bool>,
    /// Fetch the customer data associated with the connector (e.g. customer secrets / config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_customer_data: Option<bool>,
}

impl ConnectorGetV1QueryRequest {
    pub fn builder() -> ConnectorGetV1QueryRequestBuilder {
        <ConnectorGetV1QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorGetV1QueryRequestBuilder {
    fetch_user_data: Option<bool>,
    fetch_customer_data: Option<bool>,
}

impl ConnectorGetV1QueryRequestBuilder {
    pub fn fetch_user_data(mut self, value: bool) -> Self {
        self.fetch_user_data = Some(value);
        self
    }

    pub fn fetch_customer_data(mut self, value: bool) -> Self {
        self.fetch_customer_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorGetV1QueryRequest`].
    pub fn build(self) -> Result<ConnectorGetV1QueryRequest, BuildError> {
        Ok(ConnectorGetV1QueryRequest {
            fetch_user_data: self.fetch_user_data,
            fetch_customer_data: self.fetch_customer_data,
        })
    }
}
