pub use crate::prelude::*;

/// Wrapper for encrypted patch values in selective json_patch encryption.
///
/// When partial encryption mode is enabled and a patch targets an EncryptedStrField,
/// the patch value is encrypted and wrapped in this structure.
///
/// The type field acts as a discriminator to distinguish this from user data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EncryptedPatchValue {
    pub r#type: EncryptedPatchValueType,
    #[serde(default)]
    pub value: String,
}

impl EncryptedPatchValue {
    pub fn builder() -> EncryptedPatchValueBuilder {
        <EncryptedPatchValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EncryptedPatchValueBuilder {
    r#type: Option<EncryptedPatchValueType>,
    value: Option<String>,
}

impl EncryptedPatchValueBuilder {
    pub fn r#type(mut self, value: EncryptedPatchValueType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EncryptedPatchValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](EncryptedPatchValueBuilder::r#type)
    /// - [`value`](EncryptedPatchValueBuilder::value)
    pub fn build(self) -> Result<EncryptedPatchValue, BuildError> {
        Ok(EncryptedPatchValue {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
