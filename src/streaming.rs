//! SSE decoding for the byte streams returned by generated streaming methods.
//!
//! No automatic reconnection: replaying a generation POST can duplicate work
//! and charges. `json_events::<_, _, CompletionChunk>(bytes)` decodes chat/FIM
//! data. For APIs whose schema includes the SSE envelope, use `Event::envelope`.

use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use serde::de::DeserializeOwned;

/// Maximum buffered SSE event, including field names and line delimiters.
pub const MAX_EVENT_BYTES: usize = 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SSE transport: {0}")]
    Transport(String),
    #[error("SSE event exceeds {MAX_EVENT_BYTES} bytes")]
    EventTooLarge,
    #[error("Invalid UTF-8 in SSE event: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Invalid SSE JSON: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event<T> {
    pub event: String,
    pub data: T,
    pub id: String,
    pub retry: Option<u64>,
}

impl Event<String> {
    /// Decode only the JSON in the SSE `data:` field (e.g. `CompletionChunk`).
    pub fn json<T: DeserializeOwned>(&self) -> Result<T, Error> {
        Ok(serde_json::from_str(&self.data)?)
    }

    /// Decode a schema describing `{event, data, id, retry}`, such as
    /// `ConversationEvents`, `SpeechStreamEvents`, or `CompletionEvent`.
    pub fn envelope<T: DeserializeOwned>(&self) -> Result<T, Error> {
        let mut value = serde_json::json!({"event": self.event,
            "data": serde_json::from_str::<serde_json::Value>(&self.data)?});
        // Some upstream envelopes deny unknown fields. Include metadata only
        // when it was actually sent, rather than synthesizing empty values.
        if !self.id.is_empty() {
            value["id"] = self.id.clone().into();
        }
        if let Some(retry) = self.retry {
            value["retry"] = retry.into();
        }
        Ok(serde_json::from_value(value)?)
    }
}

/// Parse SSE incrementally across arbitrary byte boundaries. Handles LF, CRLF,
/// CR, a UTF-8 BOM, multiline data, comments, IDs, and Mistral's `[DONE]` marker.
/// Incomplete events at EOF are discarded as required by the SSE protocol.
pub fn events<S, E>(source: S) -> impl Stream<Item = Result<Event<String>, Error>>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: std::fmt::Display,
{
    async_stream::try_stream! {
        futures_util::pin_mut!(source);
        let mut line = Vec::new();
        let mut data = String::new();
        let mut event = String::new();
        let mut id = String::new();
        let mut retry = None;
        let mut first_line = true;
        let mut previous_cr = false;
        let mut size = 0;
        while let Some(chunk) = source.next().await {
            let chunk = chunk.map_err(|error| Error::Transport(error.to_string()))?;
            for byte in chunk {
                if previous_cr && byte == b'\n' {
                    previous_cr = false;
                    continue;
                }
                previous_cr = byte == b'\r';
                size += 1;
                if size > MAX_EVENT_BYTES {
                    Err(Error::EventTooLarge)?;
                }
                if byte != b'\n' && byte != b'\r' {
                    line.push(byte);
                    continue;
                }
                let mut text = std::str::from_utf8(&line)?;
                if first_line {
                    text = text.strip_prefix('\u{feff}').unwrap_or(text);
                    first_line = false;
                }
                if text.is_empty() {
                    size = 0;
                    if !data.is_empty() {
                        data.pop(); // final LF added by a data field
                        if data == "[DONE]" {
                            return;
                        }
                        yield Event {
                            event: if event.is_empty() { "message".into() } else { event.clone() },
                            data: std::mem::take(&mut data),
                            id: id.clone(),
                            retry,
                        };
                    }
                    event.clear();
                } else if !text.starts_with(':') {
                    let (field, value) = text.split_once(':').unwrap_or((text, ""));
                    let value = value.strip_prefix(' ').unwrap_or(value);
                    match field {
                        "data" => { data.push_str(value); data.push('\n'); }
                        "event" => event = value.into(),
                        "id" if !value.contains('\0') => id = value.into(),
                        "retry" if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) => {
                            if let Ok(value) = value.parse() { retry = Some(value); }
                        }
                        _ => {}
                    }
                }
                line.clear();
            }
        }
    }
}

/// Decode each event's JSON data into a generated type; stop on the first error.
pub fn json_events<S, E, T>(source: S) -> impl Stream<Item = Result<Event<T>, Error>>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: std::fmt::Display,
    T: DeserializeOwned,
{
    async_stream::try_stream! {
        let parsed = events(source);
        futures_util::pin_mut!(parsed);
        while let Some(event) = parsed.next().await {
            let event = event?;
            yield Event { data: event.json()?, event: event.event,
                id: event.id, retry: event.retry };
        }
    }
}
