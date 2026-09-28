fn print_menu() {
    println!("=== GAME MENU ===");
    println!("1. Buy food");
    println!("2. Hunt");
    println!("3. Trade with caravan");
    println!("4. View status");
    println!("5. Quit");
}

fn main() {
    // Temporarily call print_menu to avoid unused function warnings
    print_menu();
}
use std::io::{self, Write};

fn read_number(min: u64, max: u64) -> u64 {
    loop {
        print!("Enter a number between {} and {}: ", min, max);
        // Ensure the prompt prints before waiting for input
        let _ = io::stdout().flush();

        let mut input = String::new();
        
        // Check if read_line succeeded
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input. Please try again.");
            continue;
        }

        // Trim whitespace/newlines and attempt to parse as u64
        let trimmed = input.trim();
        let value = match trimmed.parse::<u64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a whole number.");
                continue;
            }
        };

        // Check if value is within range [min, max]
        if value >= min && value <= max {
            return value;
        } else {
            println!("Number out of range. Must be between {} and {}.", min, max);
        }
    }
}
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn roll_die(seed: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.random_range(1..=6)
}

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

/// Helper function to roll a single die with a specific seed
fn roll_die(seed: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.gen_range(1..=6)
}

/// Rolls `count` dice sequentially starting from `seed` and returns their sum.
fn roll_many(seed: u64, count: u64) -> u64 {
    let mut total = 0;
    for i in 0..count {
        total += roll_die(seed + i);
    }
    total
}
/// Returns the amount of food gained based on a d6 die roll value.
/// 
/// - Roll 1:    0 food
/// - Roll 2-3: 10 food
/// - Roll 4-5: 20 food
/// - Roll 6:   40 food
fn hunt_food(roll: u64) -> u64 {
    match roll {
        1 => 0,
        2 | 3 => 10,
        4 | 5 => 20,
        6 => 40,
        _ => 0,
    }
}
/// Returns true if the player has enough gold to cover the given cost.
fn can_afford(gold: u64, cost: u64) -> bool {
    gold >= cost
}
