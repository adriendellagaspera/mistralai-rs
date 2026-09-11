#[derive(Debug, Clone)]
pub enum AgentCreationRequestToolsItemUnion {
    FunctionTool(FunctionTool),
    WebSearchTool(WebSearchTool),
    WebSearchPremiumTool(WebSearchPremiumTool),
    CodeInterpreterTool(CodeInterpreterTool),
    ImageGenerationTool(ImageGenerationTool),
    DocumentLibraryTool(DocumentLibraryTool),
    CustomConnector(CustomConnector),
}
