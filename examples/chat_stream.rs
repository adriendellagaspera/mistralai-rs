use futures_util::StreamExt;
use mistralai::raw::types::ChatCompletionRequest;
use mistralai::{Mistral, StreamChatRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mistral = Mistral::new(std::env::var("MISTRAL_API_KEY")?);
    let raw: ChatCompletionRequest = serde_json::from_value(serde_json::json!({
        "model": "mistral-small-latest",
        "messages": [{"role": "user", "content": "Say hello in French."}]
    }))?;
    let stream = mistral.chat().stream(StreamChatRequest::from(raw)).await?;

    futures_util::pin_mut!(stream);
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        println!("{}", serde_json::to_string(chunk.raw())?);
    }
    Ok(())
}
