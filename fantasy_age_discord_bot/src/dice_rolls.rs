use rand::Rng;
use regex::Regex;
use std::collections::HashMap;

pub fn main_dice_roller(modifier: i32) -> String {
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

pub fn damage_dice_roller(input: &str) -> String {
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
    let mut rng = rand::rng();
    (0..num).map(|_| rng.random_range(1..=6)).collect()
}