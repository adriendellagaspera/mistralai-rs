use mistralai::raw::types::ChatCompletionRequest;
use mistralai::{CompleteChatRequest, Mistral};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mistral = Mistral::new(std::env::var("MISTRAL_API_KEY")?);
    let model = std::env::var("MISTRAL_MODEL").unwrap_or("mistral-small-latest".into());
    let raw: ChatCompletionRequest = serde_json::from_value(serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": "Say hello in French."}],
        "max_tokens": 64
    }))?;

    let response = mistral.chat().complete(CompleteChatRequest::from(raw)).await?;
    println!("{}", serde_json::to_string_pretty(response.raw())?);
    Ok(())
}
