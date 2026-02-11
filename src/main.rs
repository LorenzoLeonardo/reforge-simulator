use std::{io::Write, time::Duration};

use rand::distr::{Distribution, weighted::WeightedIndex};

const COST_PER_ATTEMPT: u64 = 200;

const WEIGHT_UNCOMMON: u64 = 530; // Success rate 53% of 1000
const WEIGHT_RARE: u64 = 300; // Success rate 30% of 1000
const WEIGHT_EPIC: u64 = 125; // Success rate 12.5%  of 1000
const WEIGHT_LEGENDARY: u64 = 40; // Success rate 4%  of 1000
const WEIGHT_ANCIENT: u64 = 5; // Success rate 0.5% of 1000

#[derive(Debug)]
enum Rarity {
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Ancient,
}

const RARITIES: [Rarity; 5] = [
    Rarity::Uncommon,
    Rarity::Rare,
    Rarity::Epic,
    Rarity::Legendary,
    Rarity::Ancient,
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let limit = args
        .get(1) // safely get index 1
        .and_then(|s| s.parse::<u64>().ok()) // try parse
        .unwrap_or(10); // default to 10 if missing or invalid
    let mut rng = rand::rng();

    let weights = [
        WEIGHT_UNCOMMON,
        WEIGHT_RARE,
        WEIGHT_EPIC,
        WEIGHT_LEGENDARY,
        WEIGHT_ANCIENT,
    ];
    let dist = WeightedIndex::new(weights)?;

    let mut attempts = 0;
    let mut uncommon_count = 0;
    let mut rare_count = 0;
    let mut epic_count = 0;
    let mut legendary_count = 0;
    let mut ancient_count = 0;
    while attempts < limit {
        attempts += 1;

        print!("Attempt {attempts} . . .");
        std::io::stdout().flush()?;
        std::thread::sleep(Duration::from_secs(1));

        match RARITIES[dist.sample(&mut rng)] {
            Rarity::Uncommon => {
                uncommon_count += 1_u64;
                println!(" Uncommon!");
            }
            Rarity::Rare => {
                rare_count += 1_u64;
                println!(" Rare!");
            }
            Rarity::Epic => {
                epic_count += 1_u64;
                println!(" Epic!");
            }
            Rarity::Legendary => {
                legendary_count += 1_u64;
                println!(" Legendary!");
            }
            Rarity::Ancient => {
                ancient_count += 1_u64;
                println!(" Ancient!");
                break;
            }
        }
    }

    let total_cost = attempts * COST_PER_ATTEMPT;

    println!("=== Simulation Result ===");
    println!("Cost per roll: {COST_PER_ATTEMPT} perins");
    println!("Attempts until Ancient: {attempts}");
    println!("Total cost: {total_cost} perins");
    println!();
    println!("Uncommon:  {uncommon_count}");
    println!("Rare:      {rare_count}");
    println!("Epic:      {epic_count}");
    println!("Legendary: {legendary_count}");
    println!("Ancient:   {ancient_count}");
    Ok(())
}
