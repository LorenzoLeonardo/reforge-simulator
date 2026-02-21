use std::{error::Error, io::Write};

use rand::distr::Distribution;

use crate::{ProbabilityDistribution, build_distribution};

const PERCENT100: u64 = 1000; // Using 1000 to represent 100% for better precision
const WEIGHT_UNCOMMON: u64 = 530; // Success rate 53% of 1000
const WEIGHT_RARE: u64 = 300; // Success rate 30% of 1000
const WEIGHT_EPIC: u64 = 125; // Success rate 12.5%  of 1000
const WEIGHT_LEGENDARY: u64 = 40; // Success rate 4%  of 1000
const WEIGHT_ANCIENT: u64 = 5; // Success rate 0.5% of 1000

#[derive(Debug)]
pub(crate) enum Rarity {
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Ancient,
}

pub(crate) struct ReforgeSimulationResult {
    pub attempts: u64,
    pub uncommon: u64,
    pub rare: u64,
    pub epic: u64,
    pub legendary: u64,
    pub ancient: u64,
}

fn init_rarity_weights(luck: u64) -> Vec<(ProbabilityDistribution, u64)> {
    let weights = vec![
        (
            ProbabilityDistribution::Reforging(Rarity::Uncommon),
            WEIGHT_UNCOMMON - luck,
        ), // Luck decreases the chance of Uncommon
        (
            ProbabilityDistribution::Reforging(Rarity::Rare),
            WEIGHT_RARE,
        ),
        (
            ProbabilityDistribution::Reforging(Rarity::Epic),
            WEIGHT_EPIC,
        ),
        (
            ProbabilityDistribution::Reforging(Rarity::Legendary),
            WEIGHT_LEGENDARY,
        ),
        (
            ProbabilityDistribution::Reforging(Rarity::Ancient),
            WEIGHT_ANCIENT + luck,
        ), // Luck increases the chance of Ancient
    ];

    // Ensure total weight is always 1000
    let total: u64 = weights.iter().map(|(_, w)| w).sum();
    assert_eq!(
        total, PERCENT100,
        "Total weight must be {PERCENT100}, got {total}"
    );

    weights
}

pub fn run_reforge_simulation(
    limit: u64,
    luck: u64,
) -> Result<ReforgeSimulationResult, Box<dyn Error>> {
    let mut rng = rand::rng();
    let weights = init_rarity_weights(luck);
    let dist = build_distribution(&weights)?;

    let mut result = ReforgeSimulationResult {
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

        match weights[dist.sample(&mut rng)].0 {
            ProbabilityDistribution::Reforging(Rarity::Uncommon) => {
                result.uncommon += 1;
                println!(" Uncommon!");
            }
            ProbabilityDistribution::Reforging(Rarity::Rare) => {
                result.rare += 1;
                println!(" Rare!");
            }
            ProbabilityDistribution::Reforging(Rarity::Epic) => {
                result.epic += 1;
                println!(" Epic!");
            }
            ProbabilityDistribution::Reforging(Rarity::Legendary) => {
                result.legendary += 1;
                println!(" Legendary!");
            }
            ProbabilityDistribution::Reforging(Rarity::Ancient) => {
                result.ancient += 1;
                println!(" Ancient!");
                break;
            }
            _ => panic!("Unexpected distribution type, it must be for Reforging only!"),
        }
    }

    Ok(result)
}
