use mistralai::{ChatCompletionRequest, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().with_api_key(std::env::var("MISTRAL_API_KEY")?);
    // Deserialize into a generated request type; no untyped response handling.
    let request: ChatCompletionRequest = serde_json::from_value(serde_json::json!({
        "model": std::env::var("MISTRAL_MODEL").unwrap_or("mistral-small-latest".into()),
        "messages": [{"role": "user", "content": "Say hello in French."}],
        "stream": false,
        "max_tokens": 64
    }))?;
    let response = client.chat_completion_v1_chat_completions_post(request).await?;
    println!("{:?}", response.choices);
    println!("prompt_tokens: {}", response.usage.prompt_tokens);
    println!("completion_tokens: {}", response.usage.completion_tokens);
    Ok(())
}
