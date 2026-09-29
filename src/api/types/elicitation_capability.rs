pub use crate::prelude::*;

/// Capability for elicitation operations.
///
/// Clients must support at least one mode (form or url).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ElicitationCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<FormElicitationCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<UrlElicitationCapability>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ElicitationCapability {
    pub fn builder() -> ElicitationCapabilityBuilder {
        <ElicitationCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ElicitationCapabilityBuilder {
    form: Option<FormElicitationCapability>,
    url: Option<UrlElicitationCapability>,
}

impl ElicitationCapabilityBuilder {
    pub fn form(mut self, value: FormElicitationCapability) -> Self {
        self.form = Some(value);
        self
    }

    pub fn url(mut self, value: UrlElicitationCapability) -> Self {
        self.url = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ElicitationCapability`].
    pub fn build(self) -> Result<ElicitationCapability, BuildError> {
        Ok(ElicitationCapability {
            form: self.form,
            url: self.url,
            extra: Default::default(),
        })
    }
}
