pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModelList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ModelListDataItem>>,
}

impl ModelList {
    pub fn builder() -> ModelListBuilder {
        <ModelListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelListBuilder {
    object: Option<String>,
    data: Option<Vec<ModelListDataItem>>,
}

impl ModelListBuilder {
    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn data(mut self, value: Vec<ModelListDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModelList`].
    pub fn build(self) -> Result<ModelList, BuildError> {
        Ok(ModelList {
            object: self.object,
            data: self.data,
        })
    }
}
