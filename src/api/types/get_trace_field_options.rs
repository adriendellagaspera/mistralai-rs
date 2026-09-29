pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTraceFieldOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

impl GetTraceFieldOptions {
    pub fn builder() -> GetTraceFieldOptionsBuilder {
        <GetTraceFieldOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTraceFieldOptionsBuilder {
    options: Option<Vec<String>>,
}

impl GetTraceFieldOptionsBuilder {
    pub fn options(mut self, value: Vec<String>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTraceFieldOptions`].
    pub fn build(self) -> Result<GetTraceFieldOptions, BuildError> {
        Ok(GetTraceFieldOptions {
            options: self.options,
        })
    }
}
