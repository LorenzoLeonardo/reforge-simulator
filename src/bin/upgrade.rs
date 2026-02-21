//! # Item Upgrade Simulator Binary
//!
//! A command-line tool for running item upgrade simulations.
//!
//! ## Usage
//!
//! ```sh
//! cargo run --bin upgrade [LIMIT] [TYPE]
//! ```
//!
//! ### Arguments
//!
//! * `LIMIT` - Maximum number of upgrade attempts (default: 1)
//! * `TYPE` - Item type to simulate: `--normal`, `--accessories`, or `--baruna` (required)
//!
//! ## Examples
//!
//! ```sh
//! cargo run --bin upgrade -- 100 --normal
//! cargo run --bin upgrade -- 500 --accessories
//! cargo run --bin upgrade -- 1000 --baruna
//! ```
//!
//! This will simulate upgrading an item of the specified type up to the maximum level
//! or until the attempt limit is reached.

use std::error::Error;

use reforge_simulator::upgrading::{self, ItemType};

/// Entry point for the item upgrade simulator.
///
/// Parses command-line arguments to determine the simulation parameters (attempt limit and item type),
/// then runs the upgrade simulation and displays progress and final statistics.
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    fn print_usage(program: &str) {
        eprintln!(
            "Usage: {} [LIMIT] [TYPE]\n\n  LIMIT: positive integer for maximum upgrade attempts (default: 1)\n  TYPE: --normal | --accessories | --baruna (required)",
            program
        );
        eprintln!("Examples:");
        eprintln!("  cargo run --bin upgrade -- 100 --normal");
        eprintln!("  cargo run --bin upgrade -- --baruna");
    }

    let program = args.first().map(|s| s.as_str()).unwrap_or("upgrade");

    // Parse arguments with some flexibility:
    // - If first arg starts with `--` it is treated as TYPE and LIMIT defaults to 1
    // - Otherwise, first arg is LIMIT and second arg is TYPE
    let mut limit: u64 = 1;
    let upgrade_type_opt: Option<String>;

    match args.get(1) {
        None => {
            eprintln!("No upgrade type provided.");
            print_usage(program);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No upgrade type provided",
            )));
        }
        Some(s) if s == "-h" || s == "--help" => {
            print_usage(program);
            return Ok(());
        }
        Some(s) if s.starts_with("--") => {
            // user provided type as first argument
            upgrade_type_opt = Some(s.clone());
        }
        Some(s) => match s.parse::<u64>() {
            Ok(n) => {
                limit = n;
                // try to read type from second argument
                upgrade_type_opt = match args.get(2) {
                    Some(t) => Some(t.clone()),
                    None => {
                        eprintln!("No upgrade type provided.");
                        print_usage(program);
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "No upgrade type provided",
                        )));
                    }
                };
            }
            Err(_) => {
                eprintln!("Invalid LIMIT provided: '{}'.", s);
                print_usage(program);
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid LIMIT argument",
                )));
            }
        },
    }

    let upgrade_type = upgrade_type_opt.ok_or_else(|| {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No upgrade type provided, expected --normal, --accessories or --baruna",
        ))
    })?;

    let item_type = match upgrade_type.as_str() {
        "--normal" => ItemType::Normal,
        "--accessories" => ItemType::Accessories,
        "--baruna" => ItemType::Baruna,
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "Invalid upgrade type {}, expected --normal, --accessories or --baruna",
                    upgrade_type
                ),
            )));
        }
    };
    println!(
        "\n--- Running Upgrade Simulation for {:?} Items (Max Level {}) ---\n",
        item_type,
        item_type.level_cap()
    );
    let upgrade_result = upgrading::run_upgrade_simulation(limit, &item_type)?;
    println!("\n=== {:?} Item Upgrade Simulation Result ===", item_type);
    println!("Final Item Level: {}", upgrade_result.current_level);
    println!("Total Upgrade Attempts: {}", upgrade_result.attempts);
    println!();

    Ok(())
}
