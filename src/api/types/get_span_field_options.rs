pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSpanFieldOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

impl GetSpanFieldOptions {
    pub fn builder() -> GetSpanFieldOptionsBuilder {
        <GetSpanFieldOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSpanFieldOptionsBuilder {
    options: Option<Vec<String>>,
}

impl GetSpanFieldOptionsBuilder {
    pub fn options(mut self, value: Vec<String>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSpanFieldOptions`].
    pub fn build(self) -> Result<GetSpanFieldOptions, BuildError> {
        Ok(GetSpanFieldOptions {
            options: self.options,
        })
    }
}
