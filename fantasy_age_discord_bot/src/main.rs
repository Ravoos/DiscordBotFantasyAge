use serenity::{
    all::{CreateInteractionResponseMessage, Interaction}, 
    async_trait, 
    builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse}, 
    model::{
        application::{CommandDataOptionValue, CommandOptionType},
        gateway::Ready,
    }, prelude::*
};
use anyhow::Result;
use rand::Rng;
use regex::Regex;
use dotenv::dotenv;
use std::{collections::HashMap, env};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // Check if this is a command interaction
        if let Interaction::Command(command) = interaction 
        {
            // Determine which command it is
            let response = match command.data.name.as_str() {
                "mainroll" => {
                    // Extract optional integer modifier
                    let modifier: i32 = match command.data.options.get(0).map(|opt| &opt.value) 
                    {
                        Some(CommandDataOptionValue::Integer(i)) => *i as i32,
                        _ => 0,
                    };
                    main_dice_roller(modifier)
                }
                "damageroll" => {
                    // Extract dice expression string
                    let expr: String = match command.data.options.get(0).map(|opt| &opt.value) 
                    {
                        Some(CommandDataOptionValue::String(s)) => s.clone(),
                        _ => "1d6".to_string(),
                    };
                    damage_dice_roller(&expr)
                }
                _ => "Unknown command.".to_string(),
            };

            // Send response back
            if let Err(why) = command
                .create_response(&ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(&response)
                    )
                ).await
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
    // Roll 3d6 and adds the modifier to the roll
    let rolls = roll_d6(3);
    let base_total: i32 = rolls.iter().map(|&r| r as i32).sum();
    let final_amount = base_total + modifier;

    // If there are duplicates in the roll, award stunt points based on the last die rolled
    let mut counts = HashMap::new();
    for &roll in &rolls {
        *counts.entry(roll).or_insert(0) += 1;
    }

    let has_duplicates = counts.values().any(|&count| count > 1);
    let last_roll = rolls.last().copied().unwrap_or(0);

    // Format the result
    let modifier_str = if modifier == 0 {
        String::new()
    } else if modifier > 0 {
        format!(" + {}", modifier)
    } else {
        format!(" - {}", modifier.abs())
    };

    let mut output = format!(
        "Result: {:?}{} = {}",
        rolls,
        modifier_str,
        final_amount
    );

    // Add this if we have duplicates
    if has_duplicates {
        output.push_str(&format!("\n**DOUBLES!** You gain {} stunt points!", last_roll));
    }

    output
}

fn damage_dice_roller(input: &str) -> String {
    // Regex to match Xd6+Y or Xd6-Y
    let pattern = Regex::new(r"(?i)^(\d+)d6\s*([+-]\s*\d+)?$")
        .expect("Failed to compile regex");

    // Parse the input and extract number of dice and modifier
    if let Some(caps) = pattern.captures(input)
    {
        let num_dice: u32 = caps
            .get(1)
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(1);

        // Limit dice to prevent abuse (max 100d6)
        let num_dice = num_dice.min(100);

        let modifier: i32 = caps
            .get(2)
            .and_then(|m| m.as_str().replace(' ', "").parse().ok())
            .unwrap_or(0);

        // Roll the x amount of d6 and calculate total
        let rolls = roll_d6(num_dice);
        let base_total: i32 = rolls.iter().map(|&r| r as i32).sum();
        let total = base_total + modifier;

        // Format the modifier string properly
        let modifier_str = if modifier == 0 {
            String::new()
        } else if modifier > 0 {
            format!("+{}", modifier)
        } else {
            format!("{}", modifier)
        };

        // Format and print the result
        format!(
            "You rolled {}d6{} {:?} = **{}**",
            num_dice,
            modifier_str,
            rolls,
            total
        )
    } else
    {
        String::from("Invalid format. Please use Xd6+Y or Xd6-Y, e.g., 2d6+3")
    }
}

fn roll_d6(num: u32) -> Vec<u32> {
    let mut rng = rand::rng();
    (0..num).map(|_| rng.random_range(1..=6)).collect()
}


#[tokio::main]
async fn main() -> Result<()> {
    // Load environment and initialize logger
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // Get bot token and intents
    let token = env::var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::GUILDS;

    // Create Discord client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;

    // --- Register global slash commands ---
    use serenity::model::application::Command; // <-- important import

    Command::set_global_commands(
        &client.http,
        vec!
        [
            // Main roll command
            register_main_roll_command(),
            
            // Damage roll command
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
    .await?;

    tracing::info!("Registered global commands successfully.");

    // --- Start the bot ---
    if let Err(why) = client.start().await {
        tracing::error!("Client error: {:?}", why);
    }

    Ok(())
}