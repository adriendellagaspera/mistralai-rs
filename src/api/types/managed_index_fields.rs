pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ManagedIndexFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_fields: Option<HashMap<String, ManagedIndexFieldsChunkFieldsValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_fields: Option<HashMap<String, ManagedIndexFieldsDocumentFieldsValue>>,
}

impl ManagedIndexFields {
    pub fn builder() -> ManagedIndexFieldsBuilder {
        <ManagedIndexFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManagedIndexFieldsBuilder {
    chunk_fields: Option<HashMap<String, ManagedIndexFieldsChunkFieldsValue>>,
    document_fields: Option<HashMap<String, ManagedIndexFieldsDocumentFieldsValue>>,
}

impl ManagedIndexFieldsBuilder {
    pub fn chunk_fields(
        mut self,
        value: HashMap<String, ManagedIndexFieldsChunkFieldsValue>,
    ) -> Self {
        self.chunk_fields = Some(value);
        self
    }

    pub fn document_fields(
        mut self,
        value: HashMap<String, ManagedIndexFieldsDocumentFieldsValue>,
    ) -> Self {
        self.document_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ManagedIndexFields`].
    pub fn build(self) -> Result<ManagedIndexFields, BuildError> {
        Ok(ManagedIndexFields {
            chunk_fields: self.chunk_fields,
            document_fields: self.document_fields,
        })
    }
}
