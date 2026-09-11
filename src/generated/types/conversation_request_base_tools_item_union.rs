#[derive(Debug, Clone)]
pub enum ConversationRequestBaseToolsItemUnion {
    FunctionTool(FunctionTool),
    WebSearchTool(WebSearchTool),
    WebSearchPremiumTool(WebSearchPremiumTool),
    CodeInterpreterTool(CodeInterpreterTool),
    ImageGenerationTool(ImageGenerationTool),
    DocumentLibraryTool(DocumentLibraryTool),
    CustomConnector(CustomConnector),
}
