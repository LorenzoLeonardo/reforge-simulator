//! # Reforge Simulator Binary
//!
//! A command-line tool for running reforging simulations.
//!
//! ## Usage
//!
//! ```sh
//! cargo run --bin reforge [LIMIT]
//! ```
//!
//! ### Arguments
//!
//! * `LIMIT` - Maximum number of reforge attempts (default: 1)
//!
//! ## Example
//!
//! ```sh
//! cargo run --bin reforge -- 100
//! ```
//!
//! This will run up to 100 reforging attempts and display results for each rarity tier obtained.

use std::error::Error;

use reforge_simulator::reforging::{self};

/// Entry point for the reforge simulator.
///
/// Parses command-line arguments to determine the simulation limit and runs the reforging
/// simulation, displaying results for each obtained item and final statistics.
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    fn print_usage(program: &str) {
        eprintln!(
            "Usage: {} [LIMIT]\n\n  LIMIT: positive integer for maximum reforge attempts (default: 1)",
            program
        );
        eprintln!("Example: cargo run --bin reforge -- 100");
    }

    let program = args.first().map(|s| s.as_str()).unwrap_or("reforge");

    let limit: u64 = match args.get(1) {
        None => 1,
        Some(s) if s == "-h" || s == "--help" => {
            print_usage(program);
            return Ok(());
        }
        Some(s) => match s.parse::<u64>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Invalid LIMIT provided: '{}'.", s);
                print_usage(program);
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid LIMIT argument",
                )));
            }
        },
    };

    println!("=== Simulation Starting ===");
    let result = reforging::run_reforge_simulation(limit, 0)?;

    println!("=== Simulation Result ===");
    println!("Attempts until Ancient: {}", result.attempts);
    println!();
    println!("Uncommon:  {}", result.uncommon);
    println!("Rare:      {}", result.rare);
    println!("Epic:      {}", result.epic);
    println!("Legendary: {}", result.legendary);
    println!("Ancient:   {}", result.ancient);

    Ok(())
}
