use bytes::Bytes;
use futures_util::{StreamExt, stream};
use mistralai::streaming::{Error, MAX_EVENT_BYTES, events, json_events};
use mistralai::{CompletionChunk, CompletionEvent};

fn chunks(bytes: &[u8], split: usize) -> impl futures_util::Stream<Item = Result<Bytes, String>> {
    stream::iter(vec![
        Ok(Bytes::copy_from_slice(&bytes[..split])),
        Ok(Bytes::copy_from_slice(&bytes[split..])),
    ])
}

#[tokio::test]
async fn every_split_preserves_utf8_crlf_multiline_and_metadata() {
    let body = "\u{feff}: heartbeat\r\nid: 42\r\nevent: delta\r\nretry: 250\r\ndata: bonjour\r\ndata: été\r\n\r\ndata: [DONE]\r\n\r\ndata: ignored\r\n\r\n";
    for split in 0..=body.len() {
        let result: Vec<_> = events(chunks(body.as_bytes(), split)).collect().await;
        assert_eq!(result.len(), 1, "split {split}");
        let event = result[0].as_ref().unwrap();
        assert_eq!(event.data, "bonjour\nété");
        assert_eq!(event.event, "delta");
        assert_eq!(event.id, "42");
        assert_eq!(event.retry, Some(250));
    }
}

#[tokio::test]
async fn chat_chunk_usage_is_typed_and_done_terminates() {
    let body = b"data: {\"id\":\"c1\",\"object\":\"chat.completion.chunk\",\"created\":1700000000,\"model\":\"test\",\"choices\":[],\"usage\":{\"prompt_tokens\":11,\"completion_tokens\":7,\"total_tokens\":18}}\n\ndata: [DONE]\n\n";
    let result: Vec<_> = json_events::<_, _, CompletionChunk>(chunks(body, 27))
        .collect()
        .await;
    assert_eq!(result.len(), 1);
    let chunk = &result[0].as_ref().unwrap().data;
    let usage = chunk.usage.as_ref().unwrap();
    assert_eq!(usage.prompt_tokens, 11);
    assert_eq!(usage.completion_tokens, 7);
    let raw: Vec<_> = events(chunks(body, 1)).collect().await;
    let envelope: CompletionEvent = raw[0].as_ref().unwrap().envelope().unwrap();
    assert_eq!(envelope.data.id, "c1");
}

#[tokio::test]
async fn errors_limits_and_incomplete_events_are_observable() {
    let result: Vec<_> = events(stream::iter(vec![Err::<Bytes, _>("broken pipe")]))
        .collect()
        .await;
    assert!(matches!(result[0], Err(Error::Transport(_))));

    let oversized = vec![b'x'; MAX_EVENT_BYTES + 1];
    let result: Vec<_> = events(chunks(&oversized, 1)).collect().await;
    assert!(matches!(result[0], Err(Error::EventTooLarge)));

    let result: Vec<_> = events(chunks(b"data: \xff\n\n", 2)).collect().await;
    assert!(matches!(result[0], Err(Error::Utf8(_))));

    let result: Vec<_> = json_events::<_, _, CompletionChunk>(chunks(b"data: invalid\n\n", 3))
        .collect()
        .await;
    assert!(matches!(result[0], Err(Error::Json(_))));

    let result: Vec<_> = events(chunks(b"data: incomplete", 4)).collect().await;
    assert!(result.is_empty());
}

#[tokio::test]
async fn cr_delimiters_empty_data_comments_and_id_reset() {
    let body = b"id: old\rdata: first\r\r: comment\r\rid:\rdata:\r\r";
    let result: Vec<_> = events(chunks(body, 7)).collect().await;
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].as_ref().unwrap().id, "old");
    assert_eq!(result[1].as_ref().unwrap().id, "");
    assert_eq!(result[1].as_ref().unwrap().data, "");
}
