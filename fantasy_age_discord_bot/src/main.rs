mod http_health;
mod dice_rolls;
mod fantasy_age_stunts;
mod pagnation;
mod register_commands;
use serenity::{
    all::{CreateInteractionResponseMessage, Interaction},
    async_trait,
    builder::CreateInteractionResponse,
    model::{
        application::CommandDataOptionValue,
        gateway::Ready,
    },
    prelude::*,
};
use anyhow::Result;
use std::env;
use tokio::time::{sleep, Duration};

const STUNTS_PER_PAGE: usize = 5;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(component) = interaction.clone().message_component() {
        let custom_id = component.data.custom_id.clone();

        let (title, page): (String, usize) = match custom_id.rsplit_once(':') {
            Some((t, p)) => {
                let page_num = p.parse::<usize>().unwrap_or(0);
                (t.to_string(), page_num)
            }
            None => (custom_id, 0),
        };

        let stunts: Vec<String> = if title.starts_with("Basic stunts") {
            let kind = title.split(':').last().unwrap().trim();
            fantasy_age_stunts::get_basic_stunts(kind)
        } else if title.starts_with("Class stunts") {
            let class_name = title.split(':').last().unwrap().trim();
            fantasy_age_stunts::get_stunts_for_class(class_name)
        } else {
            vec!["Unknown pagination source".to_string()]
        };

        let page = page.min(stunts.len().saturating_sub(1) / STUNTS_PER_PAGE);

        let (embed, components) =
            pagnation::build_stunt_page(&title, &stunts, page);

        let _ = component
            .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::new()
                        .add_embed(embed)
                        .components(components),
                    ),
            )
            .await;

        return;
        }


        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {

                "mainroll" => {
                    let modifier: i32 = match command.data.options.get(0).map(|opt| &opt.value) {
                        Some(CommandDataOptionValue::Integer(i)) => *i as i32,
                        _ => 0,
                    };
                    let response = dice_rolls::main_dice_roller(modifier);

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

                "damageroll" => {
                    let dice : i32 = command.data.options.iter().find_map(|opt| {
                        if opt.name == "number_of_dice" {
                            if let CommandDataOptionValue::Integer(i) = &opt.value {
                                return Some(*i as i32);
                            }
                        }
                        None
                    }).unwrap_or(1);

                    let damage_modifier : i32 = command.data.options.iter().find_map(|opt| {
                        if opt.name == "damage_modifier" {
                            if let CommandDataOptionValue::Integer(i) = &opt.value {
                                return Some(*i as i32);
                            }
                        }
                        None
                    }).unwrap_or(0);

                    let dice = dice.max(1) as u32;

                    let response = dice_rolls::damage_dice_roller(dice, damage_modifier);

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

                "basicstunts" => {
                    let basic_stunt_name: String = match command.data.options.get(0).map(|opt| &opt.value) {
                        Some(CommandDataOptionValue::String(s)) => s.clone(),
                        _ => "".to_string(),
                    };

                    let output_stunts = fantasy_age_stunts::get_basic_stunts(&basic_stunt_name);

                    let (embed, components) = pagnation::build_stunt_page(
                        &format!("Basic stunts: {}", basic_stunt_name), 
                        &output_stunts, 
                        0);

                    if let Err(why) = command.create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .add_embed(embed)
                                .components(components)
                                .ephemeral(true),
                        ),
                    ).await {
                        tracing::error!("Error sending basic stunts response: {:?}", why);
                    }
                }

                "classstunts" => {
                    let class_name: String = match command.data.options.get(0).map(|opt| &opt.value) {
                        Some(CommandDataOptionValue::String(s)) => s.clone(),
                        _ => "".to_string(),
                    };
                    let output_stunts = fantasy_age_stunts::get_stunts_for_class(&class_name);

                    let (embed, components) = pagnation::build_stunt_page(
                        &format!("Class stunts: {}", class_name), 
                        &output_stunts, 
                        0);

                    if let Err(why) = command.create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .add_embed(embed)
                                .components(components)
                                .ephemeral(true),
                        ),
                    ).await {
                        tracing::error!("Error sending basic stunts response: {:?}", why);
                    }
                }
                _ => {
                    if let Err(why) = command
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .content("Unknown command"),
                            ),
                        )
                        .await
                    {
                        tracing::error!("Error sending slash command response: {:?}", why);
                    }
                }
            };
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);
    }
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
                register_commands::register_main_roll_command(),
                register_commands::register_damage_roll_command(),
                register_commands::register_basic_stunts_command(),
                register_commands::register_class_stunts_command(),
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

