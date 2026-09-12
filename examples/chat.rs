use mistralai::{ChatRequest, Message, Mistral};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mistral = Mistral::new(std::env::var("MISTRAL_API_KEY")?);
    let request = ChatRequest::new(
        std::env::var("MISTRAL_MODEL").unwrap_or("mistral-small-latest".into()),
        [Message::user("Say hello in French.")],
    )
    .max_tokens(64);

    let response = mistral.chat().complete(request).await?;
    println!("{}", response.text().unwrap_or_default());
    println!("prompt_tokens: {}", response.raw().usage.prompt_tokens);
    println!(
        "completion_tokens: {}",
        response.raw().usage.completion_tokens
    );
    Ok(())
}
