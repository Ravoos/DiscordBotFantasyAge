use rand::Rng;
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

    let mut output = format!("Result: {:?}{} = **{}**", rolls, modifier_str, final_amount);

    if has_duplicates {
        output.push_str(&format!("\n**DOUBLES!** You gain {} stunt points!", last_roll));
    }

    output
}

pub fn damage_dice_roller(dice: u32, damage_modifier: i32) -> String {
    let num_dice = dice.min(100);

    let rolls = roll_d6(num_dice);
    let base_total: i32 = rolls.iter().map(|&x| x as i32).sum();
    let final_amount = base_total + damage_modifier;

    let modifier_str = match damage_modifier {
        0 => String::new(),
        m if m > 0 => format!(" + {}", m),
        m => m.to_string(),
    };

    format!("RESULT: {}d6{} {:?} = **{}**", num_dice, modifier_str, rolls, final_amount)
}

fn roll_d6(num: u32) -> Vec<u32> {
    let mut rng = rand::rng();
    (0..num).map(|_| rng.random_range(1..=6)).collect()
}