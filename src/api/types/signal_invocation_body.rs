pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SignalInvocationBody {
    /// The name of the signal to send
    #[serde(default)]
    pub name: String,
    /// Input data for the signal, matching its schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<HashMap<String, serde_json::Value>>,
}

impl SignalInvocationBody {
    pub fn builder() -> SignalInvocationBodyBuilder {
        <SignalInvocationBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignalInvocationBodyBuilder {
    name: Option<String>,
    input: Option<HashMap<String, serde_json::Value>>,
}

impl SignalInvocationBodyBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn input(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignalInvocationBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](SignalInvocationBodyBuilder::name)
    pub fn build(self) -> Result<SignalInvocationBody, BuildError> {
        Ok(SignalInvocationBody {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            input: self.input,
        })
    }
}
