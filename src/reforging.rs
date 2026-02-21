//! # Reforging Module
//!
//! This module implements a reforging simulation system where items are randomly assigned
//! different rarity tiers. The reforging system uses a luck mechanic that shifts probabilities
//! towards higher rarities.
//!
//! ## Rarity Distribution
//!
//! - **Uncommon (53%)**: Most common outcome, reduced by luck
//! - **Rare (30%)**: Less common than Uncommon
//! - **Epic (12.5%)**: Rare outcome
//! - **Legendary (4%)**: Very rare outcome
//! - **Ancient (0.5%)**: Rarest outcome, increased by luck
//!
//! ## Luck System
//!
//! The luck parameter (0-530) shifts probabilities:
//! - Decreases the chance of getting Uncommon items
//! - Increases the chance of getting Ancient items
//! - Does not affect Rare, Epic, or Legendary probabilities

use std::{error::Error, io::Write};

use rand::distr::Distribution;

use crate::{ProbabilityDistribution, build_distribution};

/// Represents 100% probability with 0.1% precision (1000 = 100%)
const PERCENT100: u64 = 1000;
/// Weight for Uncommon rarity: 53% base probability
const WEIGHT_UNCOMMON: u64 = 530;
/// Weight for Rare rarity: 30% probability
const WEIGHT_RARE: u64 = 300;
/// Weight for Epic rarity: 12.5% probability
const WEIGHT_EPIC: u64 = 125;
/// Weight for Legendary rarity: 4% probability
const WEIGHT_LEGENDARY: u64 = 40;
/// Weight for Ancient rarity: 0.5% base probability
const WEIGHT_ANCIENT: u64 = 5;

/// Enumeration of possible item rarity tiers in the reforging system.
///
/// Each rarity tier has an associated probability that determines how often items
/// receive that rarity during a reforging simulation.
#[derive(Debug)]
pub enum Rarity {
    /// Uncommon rarity tier (53% base probability)
    Uncommon,
    /// Rare rarity tier (30% probability)
    Rare,
    /// Epic rarity tier (12.5% probability)
    Epic,
    /// Legendary rarity tier (4% probability)
    Legendary,
    /// Ancient rarity tier (0.5% base probability, increases with luck)
    Ancient,
}

/// Results from a reforging simulation.
///
/// This struct contains statistics about a completed reforging simulation,
/// including the total number of attempts and the count of each rarity tier obtained.
#[derive(Debug)]
pub struct ReforgeSimulationResult {
    /// Total number of reforging attempts made
    pub attempts: u64,
    /// Count of Uncommon items obtained
    pub uncommon: u64,
    /// Count of Rare items obtained
    pub rare: u64,
    /// Count of Epic items obtained
    pub epic: u64,
    /// Count of Legendary items obtained
    pub legendary: u64,
    /// Count of Ancient items obtained
    pub ancient: u64,
}

/// Initializes the rarity weight distribution based on luck parameter.
///
/// Creates a weighted probability distribution for rarity tiers, adjusting probabilities
/// according to the luck value. Higher luck values increase the likelihood of obtaining
/// ancient (and rarer) items.
///
/// # Arguments
///
/// * `luck` - Luck value (0-530) that modifies probabilities. Each point of luck:
///   - Decreases Uncommon weight by 1
///   - Increases Ancient weight by 1
///
/// # Panics
///
/// Panics if the total weight does not equal exactly 1000.
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

/// Executes a reforging simulation up to a specified limit or until an Ancient item is obtained.
///
/// This function simulates the reforging process by repeatedly drawing from a weighted probability
/// distribution of rarity tiers. The simulation continues until either:
/// - An Ancient item is obtained (simulation succeeds early)
/// - The limit number of attempts is reached
///
/// # Arguments
///
/// * `limit` - Maximum number of reforging attempts to perform
/// * `luck` - Luck value (0-530) that increases chances of rare items
///
/// # Returns
///
/// A [`Result`] containing [`ReforgeSimulationResult`] with final statistics, or a [`Box<dyn Error>`]
/// if the probability distribution fails to build.
///
/// # Example
///
/// ```no_run
/// use reforge_simulator::reforging;
///
/// let result = reforging::run_reforge_simulation(100, 0)?;
/// println!("Got Ancient item in {} attempts", result.attempts);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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
