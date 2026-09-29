pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OcrImageObject {
    /// X coordinate of bottom-right corner of the extracted image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_right_x: Option<i64>,
    /// Y coordinate of bottom-right corner of the extracted image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_right_y: Option<i64>,
    /// Image ID for extracted image in a page
    #[serde(default)]
    pub id: String,
    /// Annotation of the extracted image in json str
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_annotation: Option<String>,
    /// Base64 string of the extracted image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_base64: Option<String>,
    /// X coordinate of top-left corner of the extracted image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_left_x: Option<i64>,
    /// Y coordinate of top-left corner of the extracted image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_left_y: Option<i64>,
}

impl OcrImageObject {
    pub fn builder() -> OcrImageObjectBuilder {
        <OcrImageObjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrImageObjectBuilder {
    bottom_right_x: Option<i64>,
    bottom_right_y: Option<i64>,
    id: Option<String>,
    image_annotation: Option<String>,
    image_base64: Option<String>,
    top_left_x: Option<i64>,
    top_left_y: Option<i64>,
}

impl OcrImageObjectBuilder {
    pub fn bottom_right_x(mut self, value: i64) -> Self {
        self.bottom_right_x = Some(value);
        self
    }

    pub fn bottom_right_y(mut self, value: i64) -> Self {
        self.bottom_right_y = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn image_annotation(mut self, value: impl Into<String>) -> Self {
        self.image_annotation = Some(value.into());
        self
    }

    pub fn image_base64(mut self, value: impl Into<String>) -> Self {
        self.image_base64 = Some(value.into());
        self
    }

    pub fn top_left_x(mut self, value: i64) -> Self {
        self.top_left_x = Some(value);
        self
    }

    pub fn top_left_y(mut self, value: i64) -> Self {
        self.top_left_y = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrImageObject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OcrImageObjectBuilder::id)
    pub fn build(self) -> Result<OcrImageObject, BuildError> {
        Ok(OcrImageObject {
            bottom_right_x: self.bottom_right_x,
            bottom_right_y: self.bottom_right_y,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            image_annotation: self.image_annotation,
            image_base64: self.image_base64,
            top_left_x: self.top_left_x,
            top_left_y: self.top_left_y,
        })
    }
}
