mod chat;
mod error;
mod ocr;

pub use chat::{Chat, ChatRequest, ChatResponse, Message};
pub use error::SdkError;
pub use ocr::{Ocr, OcrRequest, OcrResponse};

use crate::generated::client::HttpClient;

/// Idiomatic entry point mirroring the resource-oriented Mistral SDKs.
#[derive(Clone)]
pub struct Mistral {
    raw: HttpClient,
}

impl Mistral {
    /// Create a client for the default Mistral API endpoint.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            raw: HttpClient::new().with_api_key(api_key),
        }
    }

    /// Override the API endpoint, primarily for compatible endpoints and tests.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.raw = self.raw.with_base_url(base_url);
        self
    }

    /// Chat completion operations.
    pub fn chat(&self) -> Chat<'_> {
        Chat::new(&self.raw)
    }

    /// OCR operations.
    pub fn ocr(&self) -> Ocr<'_> {
        Ocr::new(&self.raw)
    }

    /// Escape hatch to the complete generated OpenAPI client.
    pub fn raw(&self) -> &HttpClient {
        &self.raw
    }
}
