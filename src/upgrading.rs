//! # Upgrading Module
//!
//! This module implements an item upgrading simulation system where items progress through
//! multiple upgrade levels with decreasing success rates at higher levels.
//!
//! ## Item Types
//!
//! Three different item types are supported, each with distinct upgrade rate tables:
//! - **Accessories**: 20 level progression with custom rates
//! - **Baruna**: 20 level progression with higher success rates than Accessories
//! - **Normal**: 10 level progression with standard rates
//!
//! ## Upgrade Success Rates
//!
//! Success rates decrease as items reach higher levels, making late-stage upgrades progressively harder.
//! All rates are stored as weights out of 1000 (0.1% precision).

use std::{error::Error, io::Write};

use rand::distr::Distribution;

use crate::{ProbabilityDistribution, build_distribution};

/// Represents 100% probability with 0.1% precision (1000 = 100%)
const PERCENT100: u64 = 1000;

/// Upgrade success rates for Accessories items (levels 0-19)
///
/// Accessories have 20 upgrade levels with rates from 100% (early levels) down to 0.1% (level 19->20).
const ACCESSORIES_UPGRADE_RATES: [u64; 20] = [
    PERCENT100, // 0->1: 100%
    PERCENT100, // 1->2: 100%
    630,        // 2->3: 63%
    450,        // 3->4: 45%
    330,        // 4->5: 33%
    260,        // 5->6: 26%
    210,        // 6->7: 21%
    170,        // 7->8: 17%
    140,        // 8->9: 14%
    110,        // 9->10: 11%
    90,         // 10->11: 9%
    80,         // 11->12: 8%
    60,         // 12->13: 6%
    50,         // 13->14: 5%
    40,         // 14->15: 4%
    30,         // 15->16: 3%
    20,         // 16->17: 2%
    10,         // 17->18: 1%
    7,          // 18->19: 0.7%
    1,          // 19->20: 0.1%
];

/// Upgrade success rates for Normal set items (levels 0-9)
///
/// Normal items have 10 upgrade levels with rates from 100% (early levels) down to 2% (level 9->10).
const NORMAL_SET_UPGRADE_RATES: [u64; 10] = [
    PERCENT100, // +0 -> +1 : 100%
    PERCENT100, // +1 -> +2 : 100%
    700,        // +2 -> +3 : ~70%
    500,        // +3 -> +4 : ~50%
    400,        // +4 -> +5 : ~40%
    300,        // +5 -> +6 : ~30%
    200,        // +6 -> +7 : ~20%
    100,        // +7 -> +8 : ~10%
    50,         // +8 -> +9 : ~5%
    20,         // +9 -> +10: ~2%
];

/// Upgrade success rates for Baruna items (levels 0-19)
///
/// Baruna items have 20 upgrade levels with generally higher success rates than Accessories,
/// ranging from 100% (early levels) down to 1% (level 19->20).
const BARUNA_UPGRADE_RATES: [u64; 20] = [
    PERCENT100, // 0->1 : 100%
    PERCENT100, // 1->2 : 100%
    900,        // 2->3 : 90%
    800,        // 3->4 : 80%
    700,        // 4->5 : 70%
    600,        // 5->6 : 60%
    500,        // 6->7 : 50%
    420,        // 7->8 : 42%
    350,        // 8->9 : 35%
    300,        // 9->10: 30%
    250,        // 10->11: 25%
    200,        // 11->12: 20%
    150,        // 12->13: 15%
    120,        // 13->14: 12%
    90,         // 14->15: 9%
    70,         // 15->16: 7%
    50,         // 16->17: 5%
    30,         // 17->18: 3%
    20,         // 18->19: 2%
    10,         // 19->20: 1%
];

/// Represents the outcome of an upgrade attempt.
#[derive(Debug)]
pub enum Upgrade {
    /// Upgrade attempt succeeded, item advances to next level
    Success,
    /// Upgrade attempt failed, item remains at current level
    Failure,
}

/// Results from an upgrade simulation.
///
/// This struct contains statistics about a completed upgrade simulation,
/// tracking the number of attempts made and the final item level achieved.
#[derive(Debug)]
pub struct UpgradeSimulationResult {
    /// Total number of upgrade attempts made
    pub attempts: u64,
    /// Final item level achieved after all attempts
    pub current_level: u64,
}

/// Enumeration of different item types in the upgrade system.
///
/// Each item type has different upgrade mechanics, success rates, and maximum levels.
/// This allows simulation of various equipment upgrade systems within a game.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ItemType {
    /// Accessories: 20 level cap with custom upgrade rates
    Accessories,
    /// Baruna: 20 level cap with favorable upgrade rates
    Baruna,
    /// Normal: 10 level cap with standard upgrade rates
    Normal,
}

impl ItemType {
    /// Returns the maximum upgrade level for this item type.
    ///
    /// # Returns
    ///
    /// * `Accessories` and `Baruna`: 20
    /// * `Normal`: 10
    pub fn level_cap(&self) -> u64 {
        match self {
            ItemType::Accessories => 20,
            ItemType::Normal => 10,
            ItemType::Baruna => 20,
        }
    }
}

/// Retrieves the upgrade success rate for the current item level.
///
/// Looks up the success rate from the appropriate rate table based on the item type
/// and current level.
///
/// # Arguments
///
/// * `current_level` - The current upgrade level of the item (must be < level_cap)
/// * `item_type` - The type of item (determines which rate table to use)
///
/// # Returns
///
/// The success rate as a weight out of 1000 (e.g., 500 = 50% success).
///
/// # Panics
///
/// Panics if `current_level` is >= the item's level cap.
fn get_upgrade_success_rate(current_level: u64, item_type: &ItemType) -> u64 {
    let level_cap = item_type.level_cap();
    if current_level >= level_cap {
        panic!("Cannot upgrade beyond level {level_cap}");
    }

    match item_type {
        ItemType::Accessories => ACCESSORIES_UPGRADE_RATES[current_level as usize],
        ItemType::Normal => NORMAL_SET_UPGRADE_RATES[current_level as usize],
        ItemType::Baruna => BARUNA_UPGRADE_RATES[current_level as usize],
    }
}

/// Initializes upgrade outcome weights (success/failure) for the current level.
///
/// Creates a weighted probability distribution representing success and failure chances
/// for upgrading at the current item level.
///
/// # Arguments
///
/// * `current_level` - The current upgrade level of the item
/// * `item_type` - The type of item (determines success rate)
///
/// # Returns
///
/// A vector of weighted outcomes where weights sum to exactly 1000.
///
/// # Panics
///
/// Panics if the total weight does not equal exactly 1000.
fn init_upgrade_weights(
    current_level: u64,
    item_type: &ItemType,
) -> Vec<(ProbabilityDistribution, u64)> {
    let success_rate = get_upgrade_success_rate(current_level, item_type);
    let weights = vec![
        (
            ProbabilityDistribution::Upgrading(Upgrade::Success),
            success_rate,
        ),
        (
            ProbabilityDistribution::Upgrading(Upgrade::Failure),
            PERCENT100 - success_rate,
        ),
    ];

    // Ensure total weight is always 1000
    let total: u64 = weights.iter().map(|(_, w)| w).sum();
    assert_eq!(
        total, PERCENT100,
        "Total weight must be {PERCENT100}, got {total}"
    );

    weights
}

/// Executes an item upgrade simulation until max level or attempt limit is reached.
///
/// This function simulates the upgrade process by repeatedly attempting to upgrade an item.
/// Starting from level 0, it draws outcomes from a weighted probability distribution that
/// varies based on the current level. The simulation continues until either:
/// - The item reaches its maximum level
/// - The limit number of attempts is reached
///
/// # Arguments
///
/// * `limit` - Maximum number of upgrade attempts to perform
/// * `item_type` - The type of item to upgrade (determines rates and max level)
///
/// # Returns
///
/// A [`Result`] containing [`UpgradeSimulationResult`] with final statistics, or a [`Box<dyn Error>`]
/// if probability distribution construction fails.
///
/// # Example
///
/// ```no_run
/// use reforge_simulator::upgrading::{self, ItemType};
///
/// let result = upgrading::run_upgrade_simulation(1000, &ItemType::Normal)?;
/// println!("Reached level {} after {} attempts", result.current_level, result.attempts);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn run_upgrade_simulation(
    limit: u64,
    item_type: &ItemType,
) -> Result<UpgradeSimulationResult, Box<dyn Error>> {
    let mut rng = rand::rng();
    let target_level = item_type.level_cap();

    let mut result = UpgradeSimulationResult {
        attempts: 0,
        current_level: 0,
    };

    while (result.current_level < target_level) && (result.attempts < limit) {
        let weights = init_upgrade_weights(result.current_level, item_type);
        let dist = build_distribution(&weights)?;
        result.attempts += 1;

        print!(
            "Attempt {} (Level {}) . . .",
            result.attempts, result.current_level
        );
        std::io::stdout().flush()?;
        match weights[dist.sample(&mut rng)].0 {
            ProbabilityDistribution::Upgrading(Upgrade::Success) => {
                result.current_level += 1;
                println!(" Success! Now Level {}", result.current_level);
            }
            ProbabilityDistribution::Upgrading(Upgrade::Failure) => {
                println!(" Failed! Still Level {}", result.current_level);
            }
            _ => panic!("Unexpected distribution type, it must be for Upgrading only!"),
        }
    }

    Ok(result)
}
