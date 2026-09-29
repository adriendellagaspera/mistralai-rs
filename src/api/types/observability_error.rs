pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ObservabilityError {
    #[serde(default)]
    pub detail: ObservabilityErrorDetail,
}

impl ObservabilityError {
    pub fn builder() -> ObservabilityErrorBuilder {
        <ObservabilityErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ObservabilityErrorBuilder {
    detail: Option<ObservabilityErrorDetail>,
}

impl ObservabilityErrorBuilder {
    pub fn detail(mut self, value: ObservabilityErrorDetail) -> Self {
        self.detail = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ObservabilityError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`detail`](ObservabilityErrorBuilder::detail)
    pub fn build(self) -> Result<ObservabilityError, BuildError> {
        Ok(ObservabilityError {
            detail: self
                .detail
                .ok_or_else(|| BuildError::missing_field("detail"))?,
        })
    }
}
