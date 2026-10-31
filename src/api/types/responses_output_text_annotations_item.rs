pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ResponsesOutputTextAnnotationsItem {
    #[serde(rename = "file_citation")]
    #[non_exhaustive]
    FileCitation {
        #[serde(default)]
        file_id: String,
        #[serde(default)]
        filename: String,
        #[serde(default)]
        index: i64,
    },

    #[serde(rename = "url_citation")]
    #[non_exhaustive]
    UrlCitation {
        #[serde(default)]
        end_index: i64,
        #[serde(default)]
        start_index: i64,
        #[serde(default)]
        title: String,
        #[serde(default)]
        url: String,
    },

    #[serde(rename = "container_file_citation")]
    #[non_exhaustive]
    ContainerFileCitation {
        #[serde(default)]
        container_id: String,
        #[serde(default)]
        end_index: i64,
        #[serde(default)]
        file_id: String,
        #[serde(default)]
        filename: String,
        #[serde(default)]
        start_index: i64,
    },

    #[serde(rename = "file_path")]
    #[non_exhaustive]
    FilePath {
        #[serde(default)]
        file_id: String,
        #[serde(default)]
        index: i64,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ResponsesOutputTextAnnotationsItem {
    pub fn file_citation(file_id: String, filename: String, index: i64) -> Self {
        Self::FileCitation {
            file_id,
            filename,
            index,
        }
    }

    pub fn url_citation(end_index: i64, start_index: i64, title: String, url: String) -> Self {
        Self::UrlCitation {
            end_index,
            start_index,
            title,
            url,
        }
    }

    pub fn container_file_citation(
        container_id: String,
        end_index: i64,
        file_id: String,
        filename: String,
        start_index: i64,
    ) -> Self {
        Self::ContainerFileCitation {
            container_id,
            end_index,
            file_id,
            filename,
            start_index,
        }
    }

    pub fn file_path(file_id: String, index: i64) -> Self {
        Self::FilePath { file_id, index }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
