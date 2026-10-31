pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateRealtimeSessionResponse {
    #[serde(default)]
    pub client_secret: ClientSecret,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<CreateRealtimeSessionResponseObject>,
    pub purpose: ClientSessionPurpose,
}

impl CreateRealtimeSessionResponse {
    pub fn builder() -> CreateRealtimeSessionResponseBuilder {
        <CreateRealtimeSessionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRealtimeSessionResponseBuilder {
    client_secret: Option<ClientSecret>,
    expires_at: Option<DateTime<FixedOffset>>,
    object: Option<CreateRealtimeSessionResponseObject>,
    purpose: Option<ClientSessionPurpose>,
}

impl CreateRealtimeSessionResponseBuilder {
    pub fn client_secret(mut self, value: ClientSecret) -> Self {
        self.client_secret = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn object(mut self, value: CreateRealtimeSessionResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn purpose(mut self, value: ClientSessionPurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRealtimeSessionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_secret`](CreateRealtimeSessionResponseBuilder::client_secret)
    /// - [`expires_at`](CreateRealtimeSessionResponseBuilder::expires_at)
    /// - [`purpose`](CreateRealtimeSessionResponseBuilder::purpose)
    pub fn build(self) -> Result<CreateRealtimeSessionResponse, BuildError> {
        Ok(CreateRealtimeSessionResponse {
            client_secret: self
                .client_secret
                .ok_or_else(|| BuildError::missing_field("client_secret"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            object: self.object,
            purpose: self
                .purpose
                .ok_or_else(|| BuildError::missing_field("purpose"))?,
        })
    }
}
