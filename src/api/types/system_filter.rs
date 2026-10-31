pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SystemFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SystemNameFilterGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<SystemParamFilterGroup>,
}

impl SystemFilter {
    pub fn builder() -> SystemFilterBuilder {
        <SystemFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemFilterBuilder {
    name: Option<SystemNameFilterGroup>,
    params: Option<SystemParamFilterGroup>,
}

impl SystemFilterBuilder {
    pub fn name(mut self, value: SystemNameFilterGroup) -> Self {
        self.name = Some(value);
        self
    }

    pub fn params(mut self, value: SystemParamFilterGroup) -> Self {
        self.params = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SystemFilter`].
    pub fn build(self) -> Result<SystemFilter, BuildError> {
        Ok(SystemFilter {
            name: self.name,
            params: self.params,
        })
    }
}
