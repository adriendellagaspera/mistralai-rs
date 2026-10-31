pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NoneAuthMethod {
    /// Headers whose values must be provided as credentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<ConnectorAuthenticationHeader>>,
}

impl NoneAuthMethod {
    pub fn builder() -> NoneAuthMethodBuilder {
        <NoneAuthMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NoneAuthMethodBuilder {
    headers: Option<Vec<ConnectorAuthenticationHeader>>,
}

impl NoneAuthMethodBuilder {
    pub fn headers(mut self, value: Vec<ConnectorAuthenticationHeader>) -> Self {
        self.headers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NoneAuthMethod`].
    pub fn build(self) -> Result<NoneAuthMethod, BuildError> {
        Ok(NoneAuthMethod {
            headers: self.headers,
        })
    }
}
