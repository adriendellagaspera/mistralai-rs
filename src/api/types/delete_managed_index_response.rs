pub use crate::prelude::*;

/// Wire representation returned when an index is deleted (id, name, status).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeleteManagedIndexResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub status: ManagedIndexStatus,
}

impl DeleteManagedIndexResponse {
    pub fn builder() -> DeleteManagedIndexResponseBuilder {
        <DeleteManagedIndexResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteManagedIndexResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    status: Option<ManagedIndexStatus>,
}

impl DeleteManagedIndexResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: ManagedIndexStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteManagedIndexResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DeleteManagedIndexResponseBuilder::id)
    /// - [`name`](DeleteManagedIndexResponseBuilder::name)
    /// - [`status`](DeleteManagedIndexResponseBuilder::status)
    pub fn build(self) -> Result<DeleteManagedIndexResponse, BuildError> {
        Ok(DeleteManagedIndexResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
