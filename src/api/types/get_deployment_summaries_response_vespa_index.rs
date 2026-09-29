pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDeploymentSummariesResponseVespaIndex {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_count: Option<i64>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl GetDeploymentSummariesResponseVespaIndex {
    pub fn builder() -> GetDeploymentSummariesResponseVespaIndexBuilder {
        <GetDeploymentSummariesResponseVespaIndexBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDeploymentSummariesResponseVespaIndexBuilder {
    document_count: Option<i64>,
    id: Option<String>,
    name: Option<String>,
}

impl GetDeploymentSummariesResponseVespaIndexBuilder {
    pub fn document_count(mut self, value: i64) -> Self {
        self.document_count = Some(value);
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

    /// Consumes the builder and constructs a [`GetDeploymentSummariesResponseVespaIndex`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetDeploymentSummariesResponseVespaIndexBuilder::id)
    /// - [`name`](GetDeploymentSummariesResponseVespaIndexBuilder::name)
    pub fn build(self) -> Result<GetDeploymentSummariesResponseVespaIndex, BuildError> {
        Ok(GetDeploymentSummariesResponseVespaIndex {
            document_count: self.document_count,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
