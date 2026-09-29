pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModelList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ModelListDataItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

impl ModelList {
    pub fn builder() -> ModelListBuilder {
        <ModelListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelListBuilder {
    data: Option<Vec<ModelListDataItem>>,
    object: Option<String>,
}

impl ModelListBuilder {
    pub fn data(mut self, value: Vec<ModelListDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ModelList`].
    pub fn build(self) -> Result<ModelList, BuildError> {
        Ok(ModelList {
            data: self.data,
            object: self.object,
        })
    }
}
