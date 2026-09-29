pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TempoTraceAttributeArrayElement {
    /// A boolean element in the array
    #[serde(rename = "boolValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<bool>,
    /// An integer element in the array
    #[serde(rename = "intValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_value: Option<String>,
    /// A string element in the array
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

impl TempoTraceAttributeArrayElement {
    pub fn builder() -> TempoTraceAttributeArrayElementBuilder {
        <TempoTraceAttributeArrayElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceAttributeArrayElementBuilder {
    bool_value: Option<bool>,
    int_value: Option<String>,
    string_value: Option<String>,
}

impl TempoTraceAttributeArrayElementBuilder {
    pub fn bool_value(mut self, value: bool) -> Self {
        self.bool_value = Some(value);
        self
    }

    pub fn int_value(mut self, value: impl Into<String>) -> Self {
        self.int_value = Some(value.into());
        self
    }

    pub fn string_value(mut self, value: impl Into<String>) -> Self {
        self.string_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceAttributeArrayElement`].
    pub fn build(self) -> Result<TempoTraceAttributeArrayElement, BuildError> {
        Ok(TempoTraceAttributeArrayElement {
            bool_value: self.bool_value,
            int_value: self.int_value,
            string_value: self.string_value,
        })
    }
}
