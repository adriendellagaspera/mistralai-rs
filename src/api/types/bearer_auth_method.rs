pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BearerAuthMethod {
    /// Optional additional headers sent with requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<ConnectorAuthenticationHeader>>,
}

impl BearerAuthMethod {
    pub fn builder() -> BearerAuthMethodBuilder {
        <BearerAuthMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BearerAuthMethodBuilder {
    headers: Option<Vec<ConnectorAuthenticationHeader>>,
}

impl BearerAuthMethodBuilder {
    pub fn headers(mut self, value: Vec<ConnectorAuthenticationHeader>) -> Self {
        self.headers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BearerAuthMethod`].
    pub fn build(self) -> Result<BearerAuthMethod, BuildError> {
        Ok(BearerAuthMethod {
            headers: self.headers,
        })
    }
}
