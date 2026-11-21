pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectorErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ConnectorErrorCode>,
    #[serde(default)]
    pub detail: String,
}

impl ConnectorErrorResponse {
    pub fn builder() -> ConnectorErrorResponseBuilder {
        <ConnectorErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorErrorResponseBuilder {
    code: Option<ConnectorErrorCode>,
    detail: Option<String>,
}

impl ConnectorErrorResponseBuilder {
    pub fn code(mut self, value: ConnectorErrorCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectorErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`detail`](ConnectorErrorResponseBuilder::detail)
    pub fn build(self) -> Result<ConnectorErrorResponse, BuildError> {
        Ok(ConnectorErrorResponse {
            code: self.code,
            detail: self
                .detail
                .ok_or_else(|| BuildError::missing_field("detail"))?,
        })
    }
}
