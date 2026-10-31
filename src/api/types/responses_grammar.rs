pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesGrammar {
    #[serde(default)]
    pub definition: String,
    pub syntax: ResponsesGrammarSyntax,
    pub r#type: ResponsesGrammarType,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesGrammar {
    pub fn builder() -> ResponsesGrammarBuilder {
        <ResponsesGrammarBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesGrammarBuilder {
    definition: Option<String>,
    syntax: Option<ResponsesGrammarSyntax>,
    r#type: Option<ResponsesGrammarType>,
}

impl ResponsesGrammarBuilder {
    pub fn definition(mut self, value: impl Into<String>) -> Self {
        self.definition = Some(value.into());
        self
    }

    pub fn syntax(mut self, value: ResponsesGrammarSyntax) -> Self {
        self.syntax = Some(value);
        self
    }

    pub fn r#type(mut self, value: ResponsesGrammarType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesGrammar`].
    /// This method will fail if any of the following fields are not set:
    /// - [`definition`](ResponsesGrammarBuilder::definition)
    /// - [`syntax`](ResponsesGrammarBuilder::syntax)
    /// - [`r#type`](ResponsesGrammarBuilder::r#type)
    pub fn build(self) -> Result<ResponsesGrammar, BuildError> {
        Ok(ResponsesGrammar {
            definition: self
                .definition
                .ok_or_else(|| BuildError::missing_field("definition"))?,
            syntax: self
                .syntax
                .ok_or_else(|| BuildError::missing_field("syntax"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            extra: Default::default(),
        })
    }
}
