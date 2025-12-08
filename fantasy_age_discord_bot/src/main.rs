mod http_health;
mod dice_rolls;
use serenity::{
    all::{CreateInteractionResponseMessage, Interaction},
    async_trait,
    builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse},
    model::{
        application::{CommandDataOptionValue, CommandOptionType},
        gateway::Ready,
    },
    prelude::*,
};
use anyhow::Result;
use std::env;
use tokio::time::{sleep, Duration};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let response = match command.data.name.as_str() {
                "mainroll" => {
                    let modifier: i32 = match command.data.options.get(0).map(|opt| &opt.value) {
                        Some(CommandDataOptionValue::Integer(i)) => *i as i32,
                        _ => 0,
                    };
                    dice_rolls::main_dice_roller(modifier)
                }
                "damageroll" => {
                    let expr: String = match command.data.options.get(0).map(|opt| &opt.value) {
                        Some(CommandDataOptionValue::String(s)) => s.clone(),
                        _ => "1d6".to_string(),
                    };
                    dice_rolls::damage_dice_roller(&expr)
                }
                _ => "Unknown command.".to_string(),
            };

            if let Err(why) = command
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(&response),
                    ),
                )
                .await
            {
                tracing::error!("Error sending slash command response: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);
    }
}

fn register_main_roll_command() -> CreateCommand {
    CreateCommand::new("mainroll")
        .description("Roll 3d6 and add a modifier")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "modifier",
                "Modifier to add to the roll",
            )
            .required(false),
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let health_task = tokio::spawn(async {
        if let Err(e) = http_health::start_health_server().await {
            eprintln!("Health server error: {}", e);
        }
    });

    tracing::info!("Spawned health server.");

    let discord_task = tokio::spawn(async {
        // Load Discord token
        let token = match env::var("DISCORD_TOKEN") {
            Ok(t) => t,
            Err(_) => {
                tracing::error!("DISCORD_TOKEN is not set — bot will idle.");
                loop {
                    sleep(Duration::from_secs(10)).await;
                }
            }
        };

        tracing::info!("DISCORD_TOKEN length: {}", token.len());

        // Load Application ID (REQUIRED for global commands)
        let application_id: u64 = match env::var("DISCORD_APPLICATION_ID") {
            Ok(id_str) => match id_str.parse() {
                Ok(id) => id,
                Err(_) => {
                    tracing::error!(
                        "DISCORD_APPLICATION_ID is not a valid u64: {:?}",
                        id_str
                    );
                    loop {
                        sleep(Duration::from_secs(10)).await;
                    }
                }
            },
            Err(_) => {
                tracing::error!("DISCORD_APPLICATION_ID is not set — bot will idle.");
                loop {
                    sleep(Duration::from_secs(10)).await;
                }
            }
        };

        let intents = GatewayIntents::GUILDS;

        // Create Discord client with retry
        let mut client = loop {
            match Client::builder(&token, intents)
                .event_handler(Handler)
                .await
            {
                Ok(c) => break c,
                Err(err) => {
                    tracing::error!(
                        "Failed to create Discord client: {:?}, retrying in 10s...",
                        err
                    );
                    sleep(Duration::from_secs(10)).await;
                }
            }
        };

        // Set the application ID so global command registration works
        client.http.set_application_id(application_id.into());

        // Register commands with retry
        loop {
            match serenity::model::application::Command::set_global_commands(
                &client.http,
                vec![
                    register_main_roll_command(),
                    CreateCommand::new("damageroll")
                        .description("Roll Xd6 + Y damage")
                        .add_option(
                            CreateCommandOption::new(
                                CommandOptionType::String,
                                "roll",
                                "Dice roll, e.g., 2d6+3",
                            )
                            .required(true),
                        ),
                ],
            )
            .await
            {
                Ok(_) => {
                    tracing::info!("Registered global commands.");
                    break;
                }
                Err(err) => {
                    tracing::error!(
                        "Failed to register global commands: {:?}, retrying in 10s...",
                        err
                    );
                    sleep(Duration::from_secs(10)).await;
                }
            }
        }

        // Start client
        tracing::info!("Starting Discord client event loop...");
        if let Err(why) = client.start().await {
            tracing::error!("Client error: {:?}", why);
            loop {
                sleep(Duration::from_secs(10)).await;
            }
        }
    });

    tokio::select! {
        _ = health_task => {
            tracing::error!("Health server exited — Cloud Run will restart the container.");
        }
        _ = discord_task => {
            tracing::error!("Discord bot crashed — container will remain alive.");
        }
    }

    Ok(())
}

