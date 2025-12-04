mod http_health;
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
use rand::Rng;
use regex::Regex;
use dotenv::dotenv;
use std::{collections::HashMap, env};
use tokio::time::{sleep, Duration};
use std::env;
use std::time::Duration;
use tokio::time::sleep;

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
                    main_dice_roller(modifier)
                }
                "damageroll" => {
                    let expr: String = match command.data.options.get(0).map(|opt| &opt.value) {
                        Some(CommandDataOptionValue::String(s)) => s.clone(),
                        _ => "1d6".to_string(),
                    };
                    damage_dice_roller(&expr)
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

fn main_dice_roller(modifier: i32) -> String {
    let rolls = roll_d6(3);
    let base_total: i32 = rolls.iter().map(|&x| x as i32).sum();
    let final_amount = base_total + modifier;

    let mut counts = HashMap::new();
    for &roll in &rolls {
        *counts.entry(roll).or_insert(0) += 1;
    }

    let has_duplicates = counts.values().any(|&count| count > 1);
    let last_roll = rolls.last().copied().unwrap_or(0);

    let modifier_str = if modifier == 0 {
        String::new()
    } else if modifier > 0 {
        format!(" + {}", modifier)
    } else {
        format!(" - {}", modifier.abs())
    };

    let mut output = format!("Result: {:?}{} = {}", rolls, modifier_str, final_amount);

    if has_duplicates {
        output.push_str(&format!("\n**DOUBLES!** You gain {} stunt points!", last_roll));
    }

    output
}

fn damage_dice_roller(input: &str) -> String {
    let pattern = Regex::new(r"(?i)^(\d+)d6\s*([+-]\s*\d+)?$").unwrap();

    if let Some(caps) = pattern.captures(input) {
        let num_dice: u32 = caps.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(1);
        let num_dice = num_dice.min(100);

        let modifier: i32 = caps
            .get(2)
            .and_then(|m| m.as_str().replace(' ', "").parse().ok())
            .unwrap_or(0);

        let rolls = roll_d6(num_dice);
        let base_total: i32 = rolls.iter().map(|&x| x as i32).sum();
        let total = base_total + modifier;

        let modifier_str = if modifier == 0 {
            String::new()
        } else if modifier > 0 {
            format!("+{}", modifier)
        } else {
            format!("{}", modifier)
        };

        format!(
            "You rolled {}d6{} {:?} = **{}**",
            num_dice, modifier_str, rolls, total
        )
    } else {
        "Invalid format. Please use Xd6+Y or Xd6-Y, e.g., 2d6+3".to_string()
    }
}

fn roll_d6(num: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..num).map(|_| rng.gen_range(1..=6)).collect()
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
