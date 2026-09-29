pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LimitsOut {
    /// Usage, rate, and job limits for the Organization.
    #[serde(default)]
    pub limits: LimitsContext,
}

impl LimitsOut {
    pub fn builder() -> LimitsOutBuilder {
        <LimitsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LimitsOutBuilder {
    limits: Option<LimitsContext>,
}

impl LimitsOutBuilder {
    pub fn limits(mut self, value: LimitsContext) -> Self {
        self.limits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LimitsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`limits`](LimitsOutBuilder::limits)
    pub fn build(self) -> Result<LimitsOut, BuildError> {
        Ok(LimitsOut {
            limits: self
                .limits
                .ok_or_else(|| BuildError::missing_field("limits"))?,
        })
    }
}
