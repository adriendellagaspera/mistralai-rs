pub use crate::prelude::*;

/// Organization identity and context for the authenticated caller.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhoamiOrganization {
    /// The organization's unique identifier.
    #[serde(default)]
    pub id: String,
    /// Whether this is a Mistral-internal organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    /// The organization kind (e.g. 'S' standard, 'C' company internal, 'D' dogfooding).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The organization's display name.
    #[serde(default)]
    pub name: String,
    /// The organization's billing tier (e.g. 'E' for Enterprise). Null when not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_tier: Option<String>,
}

impl WhoamiOrganization {
    pub fn builder() -> WhoamiOrganizationBuilder {
        <WhoamiOrganizationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhoamiOrganizationBuilder {
    id: Option<String>,
    internal: Option<bool>,
    kind: Option<String>,
    name: Option<String>,
    org_tier: Option<String>,
}

impl WhoamiOrganizationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn internal(mut self, value: bool) -> Self {
        self.internal = Some(value);
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn org_tier(mut self, value: impl Into<String>) -> Self {
        self.org_tier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WhoamiOrganization`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WhoamiOrganizationBuilder::id)
    /// - [`name`](WhoamiOrganizationBuilder::name)
    pub fn build(self) -> Result<WhoamiOrganization, BuildError> {
        Ok(WhoamiOrganization {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            internal: self.internal,
            kind: self.kind,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            org_tier: self.org_tier,
        })
    }
}
