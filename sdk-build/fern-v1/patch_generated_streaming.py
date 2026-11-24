#!/usr/bin/env python3
"""Restore conditional SSE methods omitted by Fern Rust generator 1.0.0."""
from __future__ import annotations

import sys
from pathlib import Path

METHODS = {
    "chat/mod.rs": ("ChatCompletionRequest", "get_chat_config", "/v1/chat/completions"),
    "fim/mod.rs": ("FimCompletionRequest", "get_fim_config", "/v1/fim/completions"),
}

TEMPLATE = """
    /// Streams completion chunks using the conditional SSE response.
    /// The request's `stream` field is forced to `true` for this method.
    pub async fn complete_stream(
        &self,
        body: &{request_type},
        options: Option<RequestOptions>,
    ) -> Result<
        std::pin::Pin<
            Box<
                dyn futures::Stream<
                        Item = Result<CompletionChunk, crate::error::ApiError>,
                    > + Send,
            >,
        >,
        crate::error::ApiError,
    > {{
        let mut body = body.clone();
        body.stream = Some(true);
        let url = "{url}";

        let mut request = self.http_client.post(url);
        if let Some(opts) = &options {{
            for (key, value) in &opts.additional_headers {{
                request = request.header(key.as_str(), value.as_str());
            }}
            for (key, value) in &opts.additional_query_params {{
                request = request.query(&[(key.as_str(), value.as_str())]);
            }}
        }}
        request = request.json(&body);
        let config = self.config.{config_getter}();
        let config = match &options {{
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {{
                let mut overridden = config.clone();
                if let Some(token) = &opts.token {{
                    overridden.access_token = Some(token.clone());
                }}
                overridden
            }}
            _ => config,
        }};
        for (key, value) in &config.custom_headers {{
            request = request.header(key.as_str(), value.as_str());
        }}
        request = crate::core::auth::apply_auth(&config, request, true, false);

        let response = self.http_client.send_with_retry(request).await?;
        if !response.status().is_success() {{
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }}
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");
        if !content_type
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .eq_ignore_ascii_case("text/event-stream")
        {{
            return Err(crate::error::ApiError::SseParseError(format!(
                "Expected Content-Type to be 'text/event-stream', got '{{}}'",
                content_type
            )));
        }}
        Ok(crate::core::sse::parse_sse_stream(response))
    }}
"""


def patch(root: Path) -> list[str]:
    if not root.is_dir():
        raise ValueError(f"Generated Rust source directory does not exist: {root}")
    changed = []
    for relative, (request_type, getter, url) in METHODS.items():
        path = root / "api" / "resources" / relative
        source = path.read_text()
        if "pub async fn complete_stream(" in source:
            raise ValueError(f"{relative}: complete_stream already exists; generator output changed")
        if "impl " not in source or not source.rstrip().endswith("}"):
            raise ValueError(f"{relative}: unexpected resource module shape")
        insert_at = source.rstrip().rfind("}")
        method = TEMPLATE.format(request_type=request_type, config_getter=getter, url=url)
        source = source[:insert_at] + "\n" + method + "\n" + source[insert_at:]
        path.write_text(source)
        changed.append(relative)
    return changed


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: patch_generated_streaming.py GENERATED_SRC_DIR")
    for item in patch(Path(sys.argv[1])):
        print(f"restored conditional SSE method: {item}")
