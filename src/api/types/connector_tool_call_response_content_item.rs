pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConnectorToolCallResponseContentItem {
    #[serde(rename = "text")]
    #[non_exhaustive]
    Text {
        #[serde(default)]
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
        #[serde(rename = "_meta")]
        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<HashMap<String, serde_json::Value>>,
    },

    #[serde(rename = "image")]
    #[non_exhaustive]
    Image {
        #[serde(default)]
        data: String,
        #[serde(rename = "mimeType")]
        #[serde(default)]
        mime_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
        #[serde(rename = "_meta")]
        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<HashMap<String, serde_json::Value>>,
    },

    #[serde(rename = "audio")]
    #[non_exhaustive]
    Audio {
        #[serde(default)]
        data: String,
        #[serde(rename = "mimeType")]
        #[serde(default)]
        mime_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
        #[serde(rename = "_meta")]
        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<HashMap<String, serde_json::Value>>,
    },

    #[serde(rename = "resource_link")]
    #[non_exhaustive]
    ResourceLink {
        #[serde(default)]
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default)]
        uri: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        mime_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        size: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        icons: Option<Vec<McpServerIcon>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
        #[serde(rename = "_meta")]
        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<HashMap<String, serde_json::Value>>,
    },

    #[serde(rename = "resource")]
    #[non_exhaustive]
    Resource {
        resource: EmbeddedResourceResource,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
        #[serde(rename = "_meta")]
        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<HashMap<String, serde_json::Value>>,
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
            text,
            annotations: None,
            meta: None,
        }
    }

    pub fn image(data: String, mime_type: String) -> Self {
        Self::Image {
            data,
            mime_type,
            annotations: None,
            meta: None,
        }
    }

    pub fn audio(data: String, mime_type: String) -> Self {
        Self::Audio {
            data,
            mime_type,
            annotations: None,
            meta: None,
        }
    }

    pub fn resource_link(name: String, uri: String) -> Self {
        Self::ResourceLink {
            name,
            title: None,
            uri,
            description: None,
            mime_type: None,
            size: None,
            icons: None,
            annotations: None,
            meta: None,
        }
    }

    pub fn resource(resource: EmbeddedResourceResource) -> Self {
        Self::Resource {
            resource,
            annotations: None,
            meta: None,
        }
    }

    pub fn text_with_annotations(
        text: String,
        annotations: Annotations,
        meta: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self::Text {
            text,
            annotations: Some(annotations),
            meta,
        }
    }

    pub fn text_with_meta(
        text: String,
        annotations: Option<Annotations>,
        meta: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self::Text {
            text,
            annotations,
            meta: Some(meta),
        }
    }

    pub fn image_with_annotations(
        data: String,
        mime_type: String,
        annotations: Annotations,
        meta: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self::Image {
            data,
            mime_type,
            annotations: Some(annotations),
            meta,
        }
    }

    pub fn image_with_meta(
        data: String,
        mime_type: String,
        annotations: Option<Annotations>,
        meta: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self::Image {
            data,
            mime_type,
            annotations,
            meta: Some(meta),
        }
    }

    pub fn audio_with_annotations(
        data: String,
        mime_type: String,
        annotations: Annotations,
        meta: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self::Audio {
            data,
            mime_type,
            annotations: Some(annotations),
            meta,
        }
    }

    pub fn audio_with_meta(
        data: String,
        mime_type: String,
        annotations: Option<Annotations>,
        meta: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self::Audio {
            data,
            mime_type,
            annotations,
            meta: Some(meta),
        }
    }

    pub fn resource_link_with_title(
        name: String,
        title: String,
        uri: String,
        description: Option<String>,
        mime_type: Option<String>,
        size: Option<i64>,
        icons: Option<Vec<McpServerIcon>>,
        annotations: Option<Annotations>,
        meta: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self::ResourceLink {
            name,
            title: Some(title),
            uri,
            description,
            mime_type,
            size,
            icons,
            annotations,
            meta,
        }
    }

    pub fn resource_link_with_description(
        name: String,
        title: Option<String>,
        uri: String,
        description: String,
        mime_type: Option<String>,
        size: Option<i64>,
        icons: Option<Vec<McpServerIcon>>,
        annotations: Option<Annotations>,
        meta: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self::ResourceLink {
            name,
            title,
            uri,
            description: Some(description),
            mime_type,
            size,
            icons,
            annotations,
            meta,
        }
    }

    pub fn resource_link_with_mime_type(
        name: String,
        title: Option<String>,
        uri: String,
        description: Option<String>,
        mime_type: String,
        size: Option<i64>,
        icons: Option<Vec<McpServerIcon>>,
        annotations: Option<Annotations>,
        meta: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self::ResourceLink {
            name,
            title,
            uri,
            description,
            mime_type: Some(mime_type),
            size,
            icons,
            annotations,
            meta,
        }
    }

    pub fn resource_link_with_size(
        name: String,
        title: Option<String>,
        uri: String,
        description: Option<String>,
        mime_type: Option<String>,
        size: i64,
        icons: Option<Vec<McpServerIcon>>,
        annotations: Option<Annotations>,
        meta: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self::ResourceLink {
            name,
            title,
            uri,
            description,
            mime_type,
            size: Some(size),
            icons,
            annotations,
            meta,
        }
    }

    pub fn resource_link_with_icons(
        name: String,
        title: Option<String>,
        uri: String,
        description: Option<String>,
        mime_type: Option<String>,
        size: Option<i64>,
        icons: Vec<McpServerIcon>,
        annotations: Option<Annotations>,
        meta: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self::ResourceLink {
            name,
            title,
            uri,
            description,
            mime_type,
            size,
            icons: Some(icons),
            annotations,
            meta,
        }
    }

    pub fn resource_link_with_annotations(
        name: String,
        title: Option<String>,
        uri: String,
        description: Option<String>,
        mime_type: Option<String>,
        size: Option<i64>,
        icons: Option<Vec<McpServerIcon>>,
        annotations: Annotations,
        meta: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self::ResourceLink {
            name,
            title,
            uri,
            description,
            mime_type,
            size,
            icons,
            annotations: Some(annotations),
            meta,
        }
    }

    pub fn resource_link_with_meta(
        name: String,
        title: Option<String>,
        uri: String,
        description: Option<String>,
        mime_type: Option<String>,
        size: Option<i64>,
        icons: Option<Vec<McpServerIcon>>,
        annotations: Option<Annotations>,
        meta: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self::ResourceLink {
            name,
            title,
            uri,
            description,
            mime_type,
            size,
            icons,
            annotations,
            meta: Some(meta),
        }
    }

    pub fn resource_with_annotations(
        resource: EmbeddedResourceResource,
        annotations: Annotations,
        meta: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self::Resource {
            resource,
            annotations: Some(annotations),
            meta,
        }
    }

    pub fn resource_with_meta(
        resource: EmbeddedResourceResource,
        annotations: Option<Annotations>,
        meta: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self::Resource {
            resource,
            annotations,
            meta: Some(meta),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
