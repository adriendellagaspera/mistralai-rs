pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateDocumentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, Option<UpdateDocumentRequestAttributesValue>>>,
    /// If set, the document will be automatically deleted after this date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
}

impl UpdateDocumentRequest {
    pub fn builder() -> UpdateDocumentRequestBuilder {
        <UpdateDocumentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDocumentRequestBuilder {
    name: Option<String>,
    attributes: Option<HashMap<String, Option<UpdateDocumentRequestAttributesValue>>>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl UpdateDocumentRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn attributes(
        mut self,
        value: HashMap<String, Option<UpdateDocumentRequestAttributesValue>>,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDocumentRequest`].
    pub fn build(self) -> Result<UpdateDocumentRequest, BuildError> {
        Ok(UpdateDocumentRequest {
            name: self.name,
            attributes: self.attributes,
            expires_at: self.expires_at,
        })
    }
}
