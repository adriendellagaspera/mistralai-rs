pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RegisterDeploymentRequestVespaField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_type: Option<SchemaFieldIndex>,
    #[serde(default)]
    pub multidimensional: bool,
    #[serde(default)]
    pub name: String,
    pub ranking: SchemaFieldRankingType,
    pub storage: SchemaFieldStorage,
    pub r#type: SchemaFieldDataType,
}

impl RegisterDeploymentRequestVespaField {
    pub fn builder() -> RegisterDeploymentRequestVespaFieldBuilder {
        <RegisterDeploymentRequestVespaFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterDeploymentRequestVespaFieldBuilder {
    index_type: Option<SchemaFieldIndex>,
    multidimensional: Option<bool>,
    name: Option<String>,
    ranking: Option<SchemaFieldRankingType>,
    storage: Option<SchemaFieldStorage>,
    r#type: Option<SchemaFieldDataType>,
}

impl RegisterDeploymentRequestVespaFieldBuilder {
    pub fn index_type(mut self, value: SchemaFieldIndex) -> Self {
        self.index_type = Some(value);
        self
    }

    pub fn multidimensional(mut self, value: bool) -> Self {
        self.multidimensional = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn ranking(mut self, value: SchemaFieldRankingType) -> Self {
        self.ranking = Some(value);
        self
    }

    pub fn storage(mut self, value: SchemaFieldStorage) -> Self {
        self.storage = Some(value);
        self
    }

    pub fn r#type(mut self, value: SchemaFieldDataType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RegisterDeploymentRequestVespaField`].
    /// This method will fail if any of the following fields are not set:
    /// - [`multidimensional`](RegisterDeploymentRequestVespaFieldBuilder::multidimensional)
    /// - [`name`](RegisterDeploymentRequestVespaFieldBuilder::name)
    /// - [`ranking`](RegisterDeploymentRequestVespaFieldBuilder::ranking)
    /// - [`storage`](RegisterDeploymentRequestVespaFieldBuilder::storage)
    /// - [`r#type`](RegisterDeploymentRequestVespaFieldBuilder::r#type)
    pub fn build(self) -> Result<RegisterDeploymentRequestVespaField, BuildError> {
        Ok(RegisterDeploymentRequestVespaField {
            index_type: self.index_type,
            multidimensional: self
                .multidimensional
                .ok_or_else(|| BuildError::missing_field("multidimensional"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            ranking: self
                .ranking
                .ok_or_else(|| BuildError::missing_field("ranking"))?,
            storage: self
                .storage
                .ok_or_else(|| BuildError::missing_field("storage"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
