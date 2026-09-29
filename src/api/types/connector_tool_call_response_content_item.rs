pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConnectorToolCallResponseContentItem {
    #[serde(rename = "text")]
    #[non_exhaustive]
    Text {
        #[serde(rename = "_meta")]
        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<HashMap<String, serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
        #[serde(default)]
        text: String,
    },

    #[serde(rename = "image")]
    #[non_exhaustive]
    Image {
        #[serde(rename = "_meta")]
        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<HashMap<String, serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
        #[serde(default)]
        data: String,
        #[serde(rename = "mimeType")]
        #[serde(default)]
        mime_type: String,
    },

    #[serde(rename = "audio")]
    #[non_exhaustive]
    Audio {
        #[serde(rename = "_meta")]
        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<HashMap<String, serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
        #[serde(default)]
        data: String,
        #[serde(rename = "mimeType")]
        #[serde(default)]
        mime_type: String,
    },

    #[serde(rename = "resource_link")]
    #[non_exhaustive]
    ResourceLink {
        #[serde(rename = "_meta")]
        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<HashMap<String, serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        icons: Option<Vec<McpServerIcon>>,
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        mime_type: Option<String>,
        #[serde(default)]
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        size: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default)]
        uri: String,
    },

    #[serde(rename = "resource")]
    #[non_exhaustive]
    Resource {
        #[serde(rename = "_meta")]
        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<HashMap<String, serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
        resource: EmbeddedResourceResource,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ConnectorToolCallResponseContentItem {
    pub fn text(text: String) -> Self {
        Self::Text {
            meta: None,
            annotations: None,
            text,
        }
    }

    pub fn image(data: String, mime_type: String) -> Self {
        Self::Image {
            meta: None,
            annotations: None,
            data,
            mime_type,
        }
    }

    pub fn audio(data: String, mime_type: String) -> Self {
        Self::Audio {
            meta: None,
            annotations: None,
            data,
            mime_type,
        }
    }

    pub fn resource_link(name: String, uri: String) -> Self {
        Self::ResourceLink {
            meta: None,
            annotations: None,
            description: None,
            icons: None,
            mime_type: None,
            name,
            size: None,
            title: None,
            uri,
        }
    }

    pub fn resource(resource: EmbeddedResourceResource) -> Self {
        Self::Resource {
            meta: None,
            annotations: None,
            resource,
        }
    }

    pub fn text_with_meta(
        meta: HashMap<String, serde_json::Value>,
        annotations: Option<Annotations>,
        text: String,
    ) -> Self {
        Self::Text {
            meta: Some(meta),
            annotations,
            text,
        }
    }

    pub fn text_with_annotations(
        meta: Option<HashMap<String, serde_json::Value>>,
        annotations: Annotations,
        text: String,
    ) -> Self {
        Self::Text {
            meta,
            annotations: Some(annotations),
            text,
        }
    }

    pub fn image_with_meta(
        meta: HashMap<String, serde_json::Value>,
        annotations: Option<Annotations>,
        data: String,
        mime_type: String,
    ) -> Self {
        Self::Image {
            meta: Some(meta),
            annotations,
            data,
            mime_type,
        }
    }

    pub fn image_with_annotations(
        meta: Option<HashMap<String, serde_json::Value>>,
        annotations: Annotations,
        data: String,
        mime_type: String,
    ) -> Self {
        Self::Image {
            meta,
            annotations: Some(annotations),
            data,
            mime_type,
        }
    }

    pub fn audio_with_meta(
        meta: HashMap<String, serde_json::Value>,
        annotations: Option<Annotations>,
        data: String,
        mime_type: String,
    ) -> Self {
        Self::Audio {
            meta: Some(meta),
            annotations,
            data,
            mime_type,
        }
    }

    pub fn audio_with_annotations(
        meta: Option<HashMap<String, serde_json::Value>>,
        annotations: Annotations,
        data: String,
        mime_type: String,
    ) -> Self {
        Self::Audio {
            meta,
            annotations: Some(annotations),
            data,
            mime_type,
        }
    }

    pub fn resource_link_with_meta(
        meta: HashMap<String, serde_json::Value>,
        annotations: Option<Annotations>,
        description: Option<String>,
        icons: Option<Vec<McpServerIcon>>,
        mime_type: Option<String>,
        name: String,
        size: Option<i64>,
        title: Option<String>,
        uri: String,
    ) -> Self {
        Self::ResourceLink {
            meta: Some(meta),
            annotations,
            description,
            icons,
            mime_type,
            name,
            size,
            title,
            uri,
        }
    }

    pub fn resource_link_with_annotations(
        meta: Option<HashMap<String, serde_json::Value>>,
        annotations: Annotations,
        description: Option<String>,
        icons: Option<Vec<McpServerIcon>>,
        mime_type: Option<String>,
        name: String,
        size: Option<i64>,
        title: Option<String>,
        uri: String,
    ) -> Self {
        Self::ResourceLink {
            meta,
            annotations: Some(annotations),
            description,
            icons,
            mime_type,
            name,
            size,
            title,
            uri,
        }
    }

    pub fn resource_link_with_description(
        meta: Option<HashMap<String, serde_json::Value>>,
        annotations: Option<Annotations>,
        description: String,
        icons: Option<Vec<McpServerIcon>>,
        mime_type: Option<String>,
        name: String,
        size: Option<i64>,
        title: Option<String>,
        uri: String,
    ) -> Self {
        Self::ResourceLink {
            meta,
            annotations,
            description: Some(description),
            icons,
            mime_type,
            name,
            size,
            title,
            uri,
        }
    }

    pub fn resource_link_with_icons(
        meta: Option<HashMap<String, serde_json::Value>>,
        annotations: Option<Annotations>,
        description: Option<String>,
        icons: Vec<McpServerIcon>,
        mime_type: Option<String>,
        name: String,
        size: Option<i64>,
        title: Option<String>,
        uri: String,
    ) -> Self {
        Self::ResourceLink {
            meta,
            annotations,
            description,
            icons: Some(icons),
            mime_type,
            name,
            size,
            title,
            uri,
        }
    }

    pub fn resource_link_with_mime_type(
        meta: Option<HashMap<String, serde_json::Value>>,
        annotations: Option<Annotations>,
        description: Option<String>,
        icons: Option<Vec<McpServerIcon>>,
        mime_type: String,
        name: String,
        size: Option<i64>,
        title: Option<String>,
        uri: String,
    ) -> Self {
        Self::ResourceLink {
            meta,
            annotations,
            description,
            icons,
            mime_type: Some(mime_type),
            name,
            size,
            title,
            uri,
        }
    }

    pub fn resource_link_with_size(
        meta: Option<HashMap<String, serde_json::Value>>,
        annotations: Option<Annotations>,
        description: Option<String>,
        icons: Option<Vec<McpServerIcon>>,
        mime_type: Option<String>,
        name: String,
        size: i64,
        title: Option<String>,
        uri: String,
    ) -> Self {
        Self::ResourceLink {
            meta,
            annotations,
            description,
            icons,
            mime_type,
            name,
            size: Some(size),
            title,
            uri,
        }
    }

    pub fn resource_link_with_title(
        meta: Option<HashMap<String, serde_json::Value>>,
        annotations: Option<Annotations>,
        description: Option<String>,
        icons: Option<Vec<McpServerIcon>>,
        mime_type: Option<String>,
        name: String,
        size: Option<i64>,
        title: String,
        uri: String,
    ) -> Self {
        Self::ResourceLink {
            meta,
            annotations,
            description,
            icons,
            mime_type,
            name,
            size,
            title: Some(title),
            uri,
        }
    }

    pub fn resource_with_meta(
        meta: HashMap<String, serde_json::Value>,
        annotations: Option<Annotations>,
        resource: EmbeddedResourceResource,
    ) -> Self {
        Self::Resource {
            meta: Some(meta),
            annotations,
            resource,
        }
    }

    pub fn resource_with_annotations(
        meta: Option<HashMap<String, serde_json::Value>>,
        annotations: Annotations,
        resource: EmbeddedResourceResource,
    ) -> Self {
        Self::Resource {
            meta,
            annotations: Some(annotations),
            resource,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
