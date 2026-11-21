pub use crate::prelude::*;

/// Contact metadata recorded when Amazon Connect starts a session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AmazonConnectContact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub called_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// Amazon Connect session context ID.
    #[serde(default)]
    pub context_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    pub r#type: AmazonConnectContactType,
}

impl AmazonConnectContact {
    pub fn builder() -> AmazonConnectContactBuilder {
        <AmazonConnectContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AmazonConnectContactBuilder {
    called_number: Option<String>,
    caller_number: Option<String>,
    channel: Option<String>,
    contact_arn: Option<String>,
    contact_id: Option<String>,
    context_id: Option<String>,
    instance_arn: Option<String>,
    instance_id: Option<String>,
    r#type: Option<AmazonConnectContactType>,
}

impl AmazonConnectContactBuilder {
    pub fn called_number(mut self, value: impl Into<String>) -> Self {
        self.called_number = Some(value.into());
        self
    }

    pub fn caller_number(mut self, value: impl Into<String>) -> Self {
        self.caller_number = Some(value.into());
        self
    }

    pub fn channel(mut self, value: impl Into<String>) -> Self {
        self.channel = Some(value.into());
        self
    }

    pub fn contact_arn(mut self, value: impl Into<String>) -> Self {
        self.contact_arn = Some(value.into());
        self
    }

    pub fn contact_id(mut self, value: impl Into<String>) -> Self {
        self.contact_id = Some(value.into());
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    pub fn instance_arn(mut self, value: impl Into<String>) -> Self {
        self.instance_arn = Some(value.into());
        self
    }

    pub fn instance_id(mut self, value: impl Into<String>) -> Self {
        self.instance_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: AmazonConnectContactType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AmazonConnectContact`].
    /// This method will fail if any of the following fields are not set:
    /// - [`context_id`](AmazonConnectContactBuilder::context_id)
    /// - [`r#type`](AmazonConnectContactBuilder::r#type)
    pub fn build(self) -> Result<AmazonConnectContact, BuildError> {
        Ok(AmazonConnectContact {
            called_number: self.called_number,
            caller_number: self.caller_number,
            channel: self.channel,
            contact_arn: self.contact_arn,
            contact_id: self.contact_id,
            context_id: self
                .context_id
                .ok_or_else(|| BuildError::missing_field("context_id"))?,
            instance_arn: self.instance_arn,
            instance_id: self.instance_id,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
