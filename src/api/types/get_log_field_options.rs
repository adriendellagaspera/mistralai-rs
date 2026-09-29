pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLogFieldOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

impl GetLogFieldOptions {
    pub fn builder() -> GetLogFieldOptionsBuilder {
        <GetLogFieldOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLogFieldOptionsBuilder {
    options: Option<Vec<String>>,
}

impl GetLogFieldOptionsBuilder {
    pub fn options(mut self, value: Vec<String>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLogFieldOptions`].
    pub fn build(self) -> Result<GetLogFieldOptions, BuildError> {
        Ok(GetLogFieldOptions {
            options: self.options,
        })
    }
}
