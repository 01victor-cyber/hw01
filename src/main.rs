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
