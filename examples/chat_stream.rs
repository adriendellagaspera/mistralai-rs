use futures_util::StreamExt;
use mistralai::{ChatRequest, Message, Mistral};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mistral = Mistral::new(std::env::var("MISTRAL_API_KEY")?);
    let stream = mistral
        .chat()
        .stream(ChatRequest::new(
            "mistral-small-latest",
            [Message::user("Say hello in French.")],
        ))
        .await?;

    futures_util::pin_mut!(stream);
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        println!("{chunk:?}");
        if let Some(usage) = chunk.usage {
            println!(
                "prompt={}, completion={}",
                usage.prompt_tokens, usage.completion_tokens
            );
        }
    }
    Ok(())
}
