pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TempoTraceAttributeArrayElement {
    /// A string element in the array
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    /// An integer element in the array
    #[serde(rename = "intValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_value: Option<String>,
    /// A boolean element in the array
    #[serde(rename = "boolValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<bool>,
}

impl TempoTraceAttributeArrayElement {
    pub fn builder() -> TempoTraceAttributeArrayElementBuilder {
        <TempoTraceAttributeArrayElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TempoTraceAttributeArrayElementBuilder {
    string_value: Option<String>,
    int_value: Option<String>,
    bool_value: Option<bool>,
}

impl TempoTraceAttributeArrayElementBuilder {
    pub fn string_value(mut self, value: impl Into<String>) -> Self {
        self.string_value = Some(value.into());
        self
    }

    pub fn int_value(mut self, value: impl Into<String>) -> Self {
        self.int_value = Some(value.into());
        self
    }

    pub fn bool_value(mut self, value: bool) -> Self {
        self.bool_value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TempoTraceAttributeArrayElement`].
    pub fn build(self) -> Result<TempoTraceAttributeArrayElement, BuildError> {
        Ok(TempoTraceAttributeArrayElement {
            string_value: self.string_value,
            int_value: self.int_value,
            bool_value: self.bool_value,
        })
    }
}
