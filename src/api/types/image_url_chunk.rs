pub use crate::prelude::*;

/// {"type":"image_url","image_url":"data:image/png;base64,iVBORw0"}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ImageUrlChunk {
    pub image_url: ImageUrlChunkImageUrl,
}

impl ImageUrlChunk {
    pub fn builder() -> ImageUrlChunkBuilder {
        <ImageUrlChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImageUrlChunkBuilder {
    image_url: Option<ImageUrlChunkImageUrl>,
}

impl ImageUrlChunkBuilder {
    pub fn image_url(mut self, value: ImageUrlChunkImageUrl) -> Self {
        self.image_url = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImageUrlChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`image_url`](ImageUrlChunkBuilder::image_url)
    pub fn build(self) -> Result<ImageUrlChunk, BuildError> {
        Ok(ImageUrlChunk {
            image_url: self
                .image_url
                .ok_or_else(|| BuildError::missing_field("image_url"))?,
        })
    }
}
