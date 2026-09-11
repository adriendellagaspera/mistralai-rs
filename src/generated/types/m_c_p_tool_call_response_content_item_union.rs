#[derive(Debug, Clone)]
pub enum MCPToolCallResponseContentItemUnion {
    TextContent(TextContent),
    ImageContent(ImageContent),
    AudioContent(AudioContent),
    ResourceLink(ResourceLink),
    EmbeddedResource(EmbeddedResource),
}
