pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OcrPageDimensions {
    /// Dots per inch of the page-image
    #[serde(default)]
    pub dpi: i64,
    /// Height of the image in pixels
    #[serde(default)]
    pub height: i64,
    /// Width of the image in pixels
    #[serde(default)]
    pub width: i64,
}

impl OcrPageDimensions {
    pub fn builder() -> OcrPageDimensionsBuilder {
        <OcrPageDimensionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrPageDimensionsBuilder {
    dpi: Option<i64>,
    height: Option<i64>,
    width: Option<i64>,
}

impl OcrPageDimensionsBuilder {
    pub fn dpi(mut self, value: i64) -> Self {
        self.dpi = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrPageDimensions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dpi`](OcrPageDimensionsBuilder::dpi)
    /// - [`height`](OcrPageDimensionsBuilder::height)
    /// - [`width`](OcrPageDimensionsBuilder::width)
    pub fn build(self) -> Result<OcrPageDimensions, BuildError> {
        Ok(OcrPageDimensions {
            dpi: self.dpi.ok_or_else(|| BuildError::missing_field("dpi"))?,
            height: self
                .height
                .ok_or_else(|| BuildError::missing_field("height"))?,
            width: self
                .width
                .ok_or_else(|| BuildError::missing_field("width"))?,
        })
    }
}
