use std::{error::Error, io::Write, time::Duration};

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

const RARITY_WEIGHTS: [(Rarity, u64); 5] = [
    (Rarity::Uncommon, WEIGHT_UNCOMMON),
    (Rarity::Rare, WEIGHT_RARE),
    (Rarity::Epic, WEIGHT_EPIC),
    (Rarity::Legendary, WEIGHT_LEGENDARY),
    (Rarity::Ancient, WEIGHT_ANCIENT),
];

struct SimulationResult {
    attempts: u64,
    uncommon: u64,
    rare: u64,
    epic: u64,
    legendary: u64,
    ancient: u64,
}

fn build_distribution() -> Result<WeightedIndex<u64>, Box<dyn Error>> {
    let weights: Vec<u64> = RARITY_WEIGHTS.iter().map(|(_, w)| *w).collect();
    Ok(WeightedIndex::new(weights)?)
}

fn run_simulation(limit: u64) -> Result<SimulationResult, Box<dyn Error>> {
    let mut rng = rand::rng();
    let dist = build_distribution()?;

    let mut result = SimulationResult {
        attempts: 0,
        uncommon: 0,
        rare: 0,
        epic: 0,
        legendary: 0,
        ancient: 0,
    };

    while result.attempts < limit {
        result.attempts += 1;

        print!("Attempt {} . . .", result.attempts);
        std::io::stdout().flush()?;
        std::thread::sleep(Duration::from_millis(500));

        match RARITY_WEIGHTS[dist.sample(&mut rng)].0 {
            Rarity::Uncommon => {
                result.uncommon += 1;
                println!(" Uncommon!");
            }
            Rarity::Rare => {
                result.rare += 1;
                println!(" Rare!");
            }
            Rarity::Epic => {
                result.epic += 1;
                println!(" Epic!");
            }
            Rarity::Legendary => {
                result.legendary += 1;
                println!(" Legendary!");
            }
            Rarity::Ancient => {
                result.ancient += 1;
                println!(" Ancient!");
                break;
            }
        }
    }

    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let limit = args
        .get(1) // safely get index 1
        .and_then(|s| s.parse::<u64>().ok()) // try parse
        .unwrap_or(10); // default to 10 if missing or invalid

    let result = run_simulation(limit)?;

    let total_cost = result.attempts * COST_PER_ATTEMPT;

    println!("=== Simulation Result ===");
    println!("Cost per roll: {COST_PER_ATTEMPT} perins");
    println!("Attempts until Ancient: {}", result.attempts);
    println!("Total cost: {total_cost} perins");
    println!();
    println!("Uncommon:  {}", result.uncommon);
    println!("Rare:      {}", result.rare);
    println!("Epic:      {}", result.epic);
    println!("Legendary: {}", result.legendary);
    println!("Ancient:   {}", result.ancient);
    Ok(())
}
