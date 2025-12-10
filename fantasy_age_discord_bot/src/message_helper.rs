use serenity::builder::{
    CreateInteractionResponse,
    CreateInteractionResponseMessage,
    CreateWebhookMessage,
};
use serenity::model::application::CommandInteraction;
use serenity::prelude::Context;

/// Splits long string into Discord-safe chunks (max 2000 chars)
fn split_into_discord_chunks(input: &str) -> Vec<String> {
    const LIMIT: usize = 1900;

    let mut messages = Vec::new();
    let mut current = String::new();

    for line in input.lines() {
        if current.len() + line.len() + 1 > LIMIT {
            messages.push(current);
            current = String::new();
        }
        current.push_str(line);
        current.push('\n');
    }

    if !current.is_empty() {
        messages.push(current);
    }

    messages
}

/// Sends long text by splitting it across multiple Discord messages.
pub async fn send_long_response(
    ctx: &Context,
    command: &CommandInteraction,
    text: &str,
) -> serenity::Result<()> {
    let chunks = split_into_discord_chunks(text);

    // 1) FIRST CHUNK → initial interaction response
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(&chunks[0]),
            ),
        )
        .await?;

    // 2) REMAINING CHUNKS → follow-up messages
    for chunk in chunks.iter().skip(1) {
        command
            .create_followup(
                &ctx.http,
                CreateWebhookMessage::new().content(chunk),
            )
            .await?;
    }

    Ok(())
}
