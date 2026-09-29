pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RegisterDeploymentRequestVespaField {
    #[serde(default)]
    pub name: String,
    pub r#type: SchemaFieldDataType,
    pub storage: SchemaFieldStorage,
    pub ranking: SchemaFieldRankingType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_type: Option<SchemaFieldIndex>,
    #[serde(default)]
    pub multidimensional: bool,
}

impl RegisterDeploymentRequestVespaField {
    pub fn builder() -> RegisterDeploymentRequestVespaFieldBuilder {
        <RegisterDeploymentRequestVespaFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterDeploymentRequestVespaFieldBuilder {
    name: Option<String>,
    r#type: Option<SchemaFieldDataType>,
    storage: Option<SchemaFieldStorage>,
    ranking: Option<SchemaFieldRankingType>,
    index_type: Option<SchemaFieldIndex>,
    multidimensional: Option<bool>,
}

impl RegisterDeploymentRequestVespaFieldBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: SchemaFieldDataType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn storage(mut self, value: SchemaFieldStorage) -> Self {
        self.storage = Some(value);
        self
    }

    pub fn ranking(mut self, value: SchemaFieldRankingType) -> Self {
        self.ranking = Some(value);
        self
    }

    pub fn index_type(mut self, value: SchemaFieldIndex) -> Self {
        self.index_type = Some(value);
        self
    }

    pub fn multidimensional(mut self, value: bool) -> Self {
        self.multidimensional = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RegisterDeploymentRequestVespaField`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RegisterDeploymentRequestVespaFieldBuilder::name)
    /// - [`r#type`](RegisterDeploymentRequestVespaFieldBuilder::r#type)
    /// - [`storage`](RegisterDeploymentRequestVespaFieldBuilder::storage)
    /// - [`ranking`](RegisterDeploymentRequestVespaFieldBuilder::ranking)
    /// - [`multidimensional`](RegisterDeploymentRequestVespaFieldBuilder::multidimensional)
    pub fn build(self) -> Result<RegisterDeploymentRequestVespaField, BuildError> {
        Ok(RegisterDeploymentRequestVespaField {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            storage: self
                .storage
                .ok_or_else(|| BuildError::missing_field("storage"))?,
            ranking: self
                .ranking
                .ok_or_else(|| BuildError::missing_field("ranking"))?,
            index_type: self.index_type,
            multidimensional: self
                .multidimensional
                .ok_or_else(|| BuildError::missing_field("multidimensional"))?,
        })
    }
}
