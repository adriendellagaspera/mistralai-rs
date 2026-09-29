pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentLibraryTool {
    /// Ids of the library in which to search.
    #[serde(default)]
    pub library_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_configuration: Option<ToolConfiguration>,
}

impl DocumentLibraryTool {
    pub fn builder() -> DocumentLibraryToolBuilder {
        <DocumentLibraryToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentLibraryToolBuilder {
    library_ids: Option<Vec<String>>,
    tool_configuration: Option<ToolConfiguration>,
}

impl DocumentLibraryToolBuilder {
    pub fn library_ids(mut self, value: Vec<String>) -> Self {
        self.library_ids = Some(value);
        self
    }

    pub fn tool_configuration(mut self, value: ToolConfiguration) -> Self {
        self.tool_configuration = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentLibraryTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`library_ids`](DocumentLibraryToolBuilder::library_ids)
    pub fn build(self) -> Result<DocumentLibraryTool, BuildError> {
        Ok(DocumentLibraryTool {
            library_ids: self
                .library_ids
                .ok_or_else(|| BuildError::missing_field("library_ids"))?,
            tool_configuration: self.tool_configuration,
        })
    }
}
