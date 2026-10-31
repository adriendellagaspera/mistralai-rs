pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SystemV2 {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub params: HashMap<String, SystemV2ParamsValue>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl SystemV2 {
    pub fn builder() -> SystemV2Builder {
        <SystemV2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemV2Builder {
    created_at: Option<DateTime<FixedOffset>>,
    deleted_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    name: Option<String>,
    params: Option<HashMap<String, SystemV2ParamsValue>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl SystemV2Builder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn params(mut self, value: HashMap<String, SystemV2ParamsValue>) -> Self {
        self.params = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SystemV2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](SystemV2Builder::created_at)
    /// - [`id`](SystemV2Builder::id)
    /// - [`name`](SystemV2Builder::name)
    /// - [`params`](SystemV2Builder::params)
    /// - [`updated_at`](SystemV2Builder::updated_at)
    pub fn build(self) -> Result<SystemV2, BuildError> {
        Ok(SystemV2 {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            params: self
                .params
                .ok_or_else(|| BuildError::missing_field("params"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
