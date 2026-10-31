pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateManagedIndexRequest {
    #[serde(default)]
    pub schema: ManagedIndexFields,
}

impl UpdateManagedIndexRequest {
    pub fn builder() -> UpdateManagedIndexRequestBuilder {
        <UpdateManagedIndexRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateManagedIndexRequestBuilder {
    schema: Option<ManagedIndexFields>,
}

impl UpdateManagedIndexRequestBuilder {
    pub fn schema(mut self, value: ManagedIndexFields) -> Self {
        self.schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateManagedIndexRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schema`](UpdateManagedIndexRequestBuilder::schema)
    pub fn build(self) -> Result<UpdateManagedIndexRequest, BuildError> {
        Ok(UpdateManagedIndexRequest {
            schema: self
                .schema
                .ok_or_else(|| BuildError::missing_field("schema"))?,
        })
    }
}
