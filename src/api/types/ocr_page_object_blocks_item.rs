pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum OcrPageObjectBlocksItem {
    #[serde(rename = "aside_text")]
    #[non_exhaustive]
    AsideText {
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        content: String,
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
    },

    #[serde(rename = "caption")]
    #[non_exhaustive]
    Caption {
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        content: String,
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
    },

    #[serde(rename = "code")]
    #[non_exhaustive]
    Code {
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        content: String,
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
    },

    #[serde(rename = "equation")]
    #[non_exhaustive]
    Equation {
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        content: String,
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
    },

    #[serde(rename = "footer")]
    #[non_exhaustive]
    Footer {
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        content: String,
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
    },

    #[serde(rename = "header")]
    #[non_exhaustive]
    Header {
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        content: String,
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
    },

    #[serde(rename = "image")]
    #[non_exhaustive]
    Image {
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        content: String,
        #[serde(default)]
        image_id: String,
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
    },

    #[serde(rename = "list")]
    #[non_exhaustive]
    List {
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        content: String,
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
    },

    #[serde(rename = "references")]
    #[non_exhaustive]
    References {
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        content: String,
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
    },

    #[serde(rename = "signature")]
    #[non_exhaustive]
    Signature {
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        content: String,
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
    },

    #[serde(rename = "table")]
    #[non_exhaustive]
    Table {
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        table_id: Option<String>,
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
    },

    #[serde(rename = "text")]
    #[non_exhaustive]
    Text {
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        content: String,
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
    },

    #[serde(rename = "title")]
    #[non_exhaustive]
    Title {
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        content: String,
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl OcrPageObjectBlocksItem {
    pub fn aside_text(
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::AsideText {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: None,
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn caption(
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Caption {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: None,
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn code(
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Code {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: None,
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn equation(
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Equation {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: None,
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn footer(
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Footer {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: None,
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn header(
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Header {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: None,
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn image(
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        image_id: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Image {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: None,
            content,
            image_id,
            top_left_x,
            top_left_y,
        }
    }

    pub fn list(
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::List {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: None,
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn references(
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::References {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: None,
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn signature(
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Signature {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: None,
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn table(
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Table {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: None,
            content,
            table_id: None,
            top_left_x,
            top_left_y,
        }
    }

    pub fn text(
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Text {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: None,
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn title(
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Title {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: None,
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn aside_text_with_confidence_scores(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: OcrBlockConfidenceScores,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::AsideText {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: Some(confidence_scores),
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn caption_with_confidence_scores(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: OcrBlockConfidenceScores,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Caption {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: Some(confidence_scores),
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn code_with_confidence_scores(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: OcrBlockConfidenceScores,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Code {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: Some(confidence_scores),
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn equation_with_confidence_scores(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: OcrBlockConfidenceScores,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Equation {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: Some(confidence_scores),
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn footer_with_confidence_scores(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: OcrBlockConfidenceScores,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Footer {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: Some(confidence_scores),
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn header_with_confidence_scores(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: OcrBlockConfidenceScores,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Header {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: Some(confidence_scores),
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn image_with_confidence_scores(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: OcrBlockConfidenceScores,
        content: String,
        image_id: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Image {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: Some(confidence_scores),
            content,
            image_id,
            top_left_x,
            top_left_y,
        }
    }

    pub fn list_with_confidence_scores(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: OcrBlockConfidenceScores,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::List {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: Some(confidence_scores),
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn references_with_confidence_scores(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: OcrBlockConfidenceScores,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::References {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: Some(confidence_scores),
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn signature_with_confidence_scores(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: OcrBlockConfidenceScores,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Signature {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: Some(confidence_scores),
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn table_with_confidence_scores(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: OcrBlockConfidenceScores,
        content: String,
        table_id: Option<String>,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Table {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: Some(confidence_scores),
            content,
            table_id,
            top_left_x,
            top_left_y,
        }
    }

    pub fn table_with_table_id(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: Option<OcrBlockConfidenceScores>,
        content: String,
        table_id: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Table {
            bottom_right_x,
            bottom_right_y,
            confidence_scores,
            content,
            table_id: Some(table_id),
            top_left_x,
            top_left_y,
        }
    }

    pub fn text_with_confidence_scores(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: OcrBlockConfidenceScores,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Text {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: Some(confidence_scores),
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn title_with_confidence_scores(
        bottom_right_x: i64,
        bottom_right_y: i64,
        confidence_scores: OcrBlockConfidenceScores,
        content: String,
        top_left_x: i64,
        top_left_y: i64,
    ) -> Self {
        Self::Title {
            bottom_right_x,
            bottom_right_y,
            confidence_scores: Some(confidence_scores),
            content,
            top_left_x,
            top_left_y,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
