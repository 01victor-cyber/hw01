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
/// Prints the current game status showing the day count, gold balance, and food units.
fn print_status(gold: u64, food: u64, day: u64) {
    println!("\n=== STATUS ===");
    println!("Day:  {}", day);
    println!("Gold: {}", gold);
    println!("Food: {}", food);
}
fn main() {
    println!("Welcome to Roll Many!");
    println!("Please set up your game seed.");
    let seed = read_number(1, MAX_SEED);

    let mut gold = START_GOLD;
    let mut food = 0;
    let mut day = 1;
    let mut roll_count = 0;

    loop {
        print_menu();
        let choice = read_number(1, 5);

        match choice {
            1 => {
                if !can_afford(gold, FOOD_PRICE) {
                    println!("You cannot afford any food right now!");
                } else {
                    let max_units = gold / FOOD_PRICE;
                    println!("How many units of food would you like to buy?");
                    let qty = read_number(1, max_units);
                    let total_cost = qty * FOOD_PRICE;

                    gold -= total_cost;
                    food += qty;
                    day += 1;
                    println!("Bought {} units of food for {} gold.", qty, total_cost);
                }
            }
            2 => {
                let current_seed = seed + roll_count;
                let roll = roll_die(current_seed);
                roll_count += 1;

                let gained = hunt_food(roll);
                food += gained;
                day += 1;
                println!("You rolled a {}! Gained {} food.", roll, gained);
            }
            3 => {
                let current_seed = seed + roll_count;
                let total = roll_many(current_seed, 3);
                roll_count += 3;

                let gold_gained = 2 * total;
                gold += gold_gained;
                day += 1;
                println!("Rolled 3 dice (total {}). Earned {} gold!", total, gold_gained);
            }
            4 => {
                print_status(gold, food, day);
            }
            5 => {
                println!("\nFinal Game State:");
                print_status(gold, food, day);
                println!("Thanks for playing!");
                break;
            }
            _ => unreachable!(),
        }
    }
}
