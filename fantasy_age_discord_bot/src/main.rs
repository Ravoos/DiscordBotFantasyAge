use lambda_http::{handler, lambda_runtime::{self, Context, Error}, Body, Request, Response};
use rand::Rng;
use regex::Regex;
use serde_json::{json, Value};
use std::collections::HashMap;

struct Handler;

async fn func(event: Request, _: Context) -> Result<Response<Body>, Error> {
    // Parse Discord interaction JSON
    let body: Value = serde_json::from_slice(event.body().as_ref())?;
    let command_name = body
        .get("data")
        .and_then(|d| d.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Determine command output
    let content = match command_name {
        "mainroll" => {
            // Extract optional modifier
            let modifier = body.get("data")
                .and_then(|d| d.get("options"))
                .and_then(|opts| opts.get(0))
                .and_then(|opt| opt.get("value"))
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32;

            main_dice_roller(modifier)
        }
        "damageroll" => {
            // Extract roll expression
            let expr = body.get("data")
                .and_then(|d| d.get("options"))
                .and_then(|opts| opts.get(0))
                .and_then(|opt| opt.get("value"))
                .and_then(|v| v.as_str())
                .unwrap_or("1d6");

            damage_dice_roller(expr)
        }
        _ => "Unknown command.".to_string(),
    };

    // Respond in Discord interaction format
    let resp = json!({
        "type": 4, // CHANNEL_MESSAGE_WITH_SOURCE
        "data": { "content": content }
    });

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::Text(resp.to_string()))
        .unwrap())
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
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}
