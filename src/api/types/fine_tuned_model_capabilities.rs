pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FineTunedModelCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_chat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_fim: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_calling: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fine_tuning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<bool>,
}

impl FineTunedModelCapabilities {
    pub fn builder() -> FineTunedModelCapabilitiesBuilder {
        <FineTunedModelCapabilitiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FineTunedModelCapabilitiesBuilder {
    completion_chat: Option<bool>,
    completion_fim: Option<bool>,
    function_calling: Option<bool>,
    fine_tuning: Option<bool>,
    classification: Option<bool>,
}

impl FineTunedModelCapabilitiesBuilder {
    pub fn completion_chat(mut self, value: bool) -> Self {
        self.completion_chat = Some(value);
        self
    }

    pub fn completion_fim(mut self, value: bool) -> Self {
        self.completion_fim = Some(value);
        self
    }

    pub fn function_calling(mut self, value: bool) -> Self {
        self.function_calling = Some(value);
        self
    }

    pub fn fine_tuning(mut self, value: bool) -> Self {
        self.fine_tuning = Some(value);
        self
    }

    pub fn classification(mut self, value: bool) -> Self {
        self.classification = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FineTunedModelCapabilities`].
    pub fn build(self) -> Result<FineTunedModelCapabilities, BuildError> {
        Ok(FineTunedModelCapabilities {
            completion_chat: self.completion_chat,
            completion_fim: self.completion_fim,
            function_calling: self.function_calling,
            fine_tuning: self.fine_tuning,
            classification: self.classification,
        })
    }
}
