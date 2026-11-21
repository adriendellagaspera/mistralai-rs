pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HangUpTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filler: Option<HangUpToolFiller>,
}

impl HangUpTool {
    pub fn builder() -> HangUpToolBuilder {
        <HangUpToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HangUpToolBuilder {
    filler: Option<HangUpToolFiller>,
}

impl HangUpToolBuilder {
    pub fn filler(mut self, value: HangUpToolFiller) -> Self {
        self.filler = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HangUpTool`].
    pub fn build(self) -> Result<HangUpTool, BuildError> {
        Ok(HangUpTool {
            filler: self.filler,
        })
    }
}
