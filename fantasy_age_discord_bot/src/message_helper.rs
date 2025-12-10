use serenity::builder::{
    CreateInteractionResponse,
    CreateInteractionResponseMessage,
    CreateWebhookMessage,
};
use serenity::model::application::CommandInteraction;
use serenity::prelude::Context;

fn split_discord_safe(input: &str) -> Vec<String> {
    const LIMIT: usize = 1900;
    let mut chunks = Vec::new();
    let mut current = String::new();

    for c in input.chars() {
        if current.chars().count() >= LIMIT {
            chunks.push(current);
            current = String::new();
        }
        current.push(c);
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}

/// Sends long text by splitting it across multiple Discord messages.
pub async fn send_long_response(
    ctx: &Context,
    command: &CommandInteraction,
    text: &str,
) -> serenity::Result<()> {

    // STEP 1: Defer the reply (acknowledge the interaction)
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new())
        )
        .await?;

    // STEP 2: Break text into 1900-char chunks
    let chunks = split_discord_safe(text);

    // STEP 3: First chunk is the "edit" to the deferred reply
    command
        .edit_response(
            &ctx.http,
            serenity::builder::EditInteractionResponse::new().content(chunks[0].clone())
        )
        .await?;

    // STEP 4: Remaining chunks are follow ups
    for chunk in chunks.into_iter().skip(1) {
        command
            .create_followup(
                &ctx.http,
                CreateWebhookMessage::new().content(chunk)
            )
            .await?;
    }

    Ok(())
}
