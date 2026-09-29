pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum OcrPageObjectBlocksItem {
    #[serde(rename = "aside_text")]
    #[non_exhaustive]
    AsideText {
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
    },

    #[serde(rename = "caption")]
    #[non_exhaustive]
    Caption {
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
    },

    #[serde(rename = "code")]
    #[non_exhaustive]
    Code {
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
    },

    #[serde(rename = "equation")]
    #[non_exhaustive]
    Equation {
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
    },

    #[serde(rename = "footer")]
    #[non_exhaustive]
    Footer {
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
    },

    #[serde(rename = "header")]
    #[non_exhaustive]
    Header {
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
    },

    #[serde(rename = "image")]
    #[non_exhaustive]
    Image {
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(default)]
        image_id: String,
    },

    #[serde(rename = "list")]
    #[non_exhaustive]
    List {
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
    },

    #[serde(rename = "references")]
    #[non_exhaustive]
    References {
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
    },

    #[serde(rename = "signature")]
    #[non_exhaustive]
    Signature {
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
    },

    #[serde(rename = "table")]
    #[non_exhaustive]
    Table {
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
        #[serde(skip_serializing_if = "Option::is_none")]
        table_id: Option<String>,
    },

    #[serde(rename = "text")]
    #[non_exhaustive]
    Text {
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
    },

    #[serde(rename = "title")]
    #[non_exhaustive]
    Title {
        #[serde(default)]
        top_left_x: i64,
        #[serde(default)]
        top_left_y: i64,
        #[serde(default)]
        bottom_right_x: i64,
        #[serde(default)]
        bottom_right_y: i64,
        #[serde(default)]
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence_scores: Option<OcrBlockConfidenceScores>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl OcrPageObjectBlocksItem {
    pub fn aside_text(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
    ) -> Self {
        Self::AsideText {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: None,
        }
    }

    pub fn caption(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
    ) -> Self {
        Self::Caption {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: None,
        }
    }

    pub fn code(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
    ) -> Self {
        Self::Code {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: None,
        }
    }

    pub fn equation(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
    ) -> Self {
        Self::Equation {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: None,
        }
    }

    pub fn footer(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
    ) -> Self {
        Self::Footer {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: None,
        }
    }

    pub fn header(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
    ) -> Self {
        Self::Header {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: None,
        }
    }

    pub fn image(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        image_id: String,
    ) -> Self {
        Self::Image {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: None,
            image_id,
        }
    }

    pub fn list(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
    ) -> Self {
        Self::List {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: None,
        }
    }

    pub fn references(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
    ) -> Self {
        Self::References {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: None,
        }
    }

    pub fn signature(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
    ) -> Self {
        Self::Signature {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: None,
        }
    }

    pub fn table(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
    ) -> Self {
        Self::Table {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: None,
            table_id: None,
        }
    }

    pub fn text(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
    ) -> Self {
        Self::Text {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: None,
        }
    }

    pub fn title(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
    ) -> Self {
        Self::Title {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: None,
        }
    }

    pub fn aside_text_with_confidence_scores(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: OcrBlockConfidenceScores,
    ) -> Self {
        Self::AsideText {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: Some(confidence_scores),
        }
    }

    pub fn caption_with_confidence_scores(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: OcrBlockConfidenceScores,
    ) -> Self {
        Self::Caption {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: Some(confidence_scores),
        }
    }

    pub fn code_with_confidence_scores(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: OcrBlockConfidenceScores,
    ) -> Self {
        Self::Code {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: Some(confidence_scores),
        }
    }

    pub fn equation_with_confidence_scores(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: OcrBlockConfidenceScores,
    ) -> Self {
        Self::Equation {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: Some(confidence_scores),
        }
    }

    pub fn footer_with_confidence_scores(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: OcrBlockConfidenceScores,
    ) -> Self {
        Self::Footer {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: Some(confidence_scores),
        }
    }

    pub fn header_with_confidence_scores(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: OcrBlockConfidenceScores,
    ) -> Self {
        Self::Header {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: Some(confidence_scores),
        }
    }

    pub fn image_with_confidence_scores(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: OcrBlockConfidenceScores,
        image_id: String,
    ) -> Self {
        Self::Image {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: Some(confidence_scores),
            image_id,
        }
    }

    pub fn list_with_confidence_scores(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: OcrBlockConfidenceScores,
    ) -> Self {
        Self::List {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: Some(confidence_scores),
        }
    }

    pub fn references_with_confidence_scores(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: OcrBlockConfidenceScores,
    ) -> Self {
        Self::References {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: Some(confidence_scores),
        }
    }

    pub fn signature_with_confidence_scores(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: OcrBlockConfidenceScores,
    ) -> Self {
        Self::Signature {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: Some(confidence_scores),
        }
    }

    pub fn table_with_confidence_scores(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: OcrBlockConfidenceScores,
        table_id: Option<String>,
    ) -> Self {
        Self::Table {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: Some(confidence_scores),
            table_id,
        }
    }

    pub fn table_with_table_id(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: Option<OcrBlockConfidenceScores>,
        table_id: String,
    ) -> Self {
        Self::Table {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores,
            table_id: Some(table_id),
        }
    }

    pub fn text_with_confidence_scores(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: OcrBlockConfidenceScores,
    ) -> Self {
        Self::Text {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: Some(confidence_scores),
        }
    }

    pub fn title_with_confidence_scores(
        top_left_x: i64,
        top_left_y: i64,
        bottom_right_x: i64,
        bottom_right_y: i64,
        content: String,
        confidence_scores: OcrBlockConfidenceScores,
    ) -> Self {
        Self::Title {
            top_left_x,
            top_left_y,
            bottom_right_x,
            bottom_right_y,
            content,
            confidence_scores: Some(confidence_scores),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
