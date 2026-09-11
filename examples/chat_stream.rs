use futures_util::StreamExt;
use mistralai::{ChatCompletionRequest, Client, CompletionChunk, streaming};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().with_api_key(std::env::var("MISTRAL_API_KEY")?);
    let request: ChatCompletionRequest = serde_json::from_value(serde_json::json!({
        "model": "mistral-small-latest",
        "messages": [{"role": "user", "content": "Say hello in French."}],
        "stream": true
    }))?;
    let bytes = client
        .chat_completion_v1_chat_completions_post_stream(request)
        .await?;
    let events = streaming::json_events::<_, _, CompletionChunk>(bytes);
    futures_util::pin_mut!(events);
    while let Some(event) = events.next().await {
        let chunk = event?.data;
        println!("{chunk:?}");
        if let Some(usage) = chunk.usage {
            println!("prompt={}, completion={}", usage.prompt_tokens, usage.completion_tokens);
        }
    }
    Ok(())
}
