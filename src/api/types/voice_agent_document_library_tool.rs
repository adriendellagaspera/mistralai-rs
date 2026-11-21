pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceAgentDocumentLibraryTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filler: Option<VoiceAgentDocumentLibraryToolFiller>,
    /// Ids of the library in which to search.
    #[serde(default)]
    pub library_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_configuration: Option<ToolConfiguration>,
}

impl VoiceAgentDocumentLibraryTool {
    pub fn builder() -> VoiceAgentDocumentLibraryToolBuilder {
        <VoiceAgentDocumentLibraryToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceAgentDocumentLibraryToolBuilder {
    filler: Option<VoiceAgentDocumentLibraryToolFiller>,
    library_ids: Option<Vec<String>>,
    tool_configuration: Option<ToolConfiguration>,
}

impl VoiceAgentDocumentLibraryToolBuilder {
    pub fn filler(mut self, value: VoiceAgentDocumentLibraryToolFiller) -> Self {
        self.filler = Some(value);
        self
    }

    pub fn library_ids(mut self, value: Vec<String>) -> Self {
        self.library_ids = Some(value);
        self
    }

    pub fn tool_configuration(mut self, value: ToolConfiguration) -> Self {
        self.tool_configuration = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceAgentDocumentLibraryTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`library_ids`](VoiceAgentDocumentLibraryToolBuilder::library_ids)
    pub fn build(self) -> Result<VoiceAgentDocumentLibraryTool, BuildError> {
        Ok(VoiceAgentDocumentLibraryTool {
            filler: self.filler,
            library_ids: self
                .library_ids
                .ok_or_else(|| BuildError::missing_field("library_ids"))?,
            tool_configuration: self.tool_configuration,
        })
    }
}
