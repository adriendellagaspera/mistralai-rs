#[derive(Debug, Clone)]
pub enum AgentToolsItemUnion {
    FunctionTool(FunctionTool),
    WebSearchTool(WebSearchTool),
    WebSearchPremiumTool(WebSearchPremiumTool),
    CodeInterpreterTool(CodeInterpreterTool),
    ImageGenerationTool(ImageGenerationTool),
    DocumentLibraryTool(DocumentLibraryTool),
    CustomConnector(CustomConnector),
}
