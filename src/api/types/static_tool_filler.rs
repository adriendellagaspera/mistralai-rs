pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StaticToolFiller {
    #[serde(default)]
    pub text: String,
}

impl StaticToolFiller {
    pub fn builder() -> StaticToolFillerBuilder {
        <StaticToolFillerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StaticToolFillerBuilder {
    text: Option<String>,
}

impl StaticToolFillerBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StaticToolFiller`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](StaticToolFillerBuilder::text)
    pub fn build(self) -> Result<StaticToolFiller, BuildError> {
        Ok(StaticToolFiller {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
