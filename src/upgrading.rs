use std::{error::Error, io::Write};

use rand::distr::Distribution;

use crate::{ProbabilityDistribution, build_distribution};

const PERCENT100: u64 = 1000; // Using 1000 to represent 100% for better precision

// Upgrade success rates (as percentages out of 1000)
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

#[derive(Debug)]
pub(crate) enum Upgrade {
    Success,
    Failure,
}

pub(crate) struct UpgradeSimulationResult {
    pub attempts: u64,
    pub current_level: u64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum ItemType {
    Accessories,
    Baruna,
    Normal,
}

impl ItemType {
    pub fn level_cap(&self) -> u64 {
        match self {
            ItemType::Accessories => 20,
            ItemType::Normal => 10,
            ItemType::Baruna => 20,
        }
    }
}

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

pub fn run_upgrade_simulation(
    item_type: &ItemType,
) -> Result<UpgradeSimulationResult, Box<dyn Error>> {
    let mut rng = rand::rng();
    let target_level = item_type.level_cap();

    let mut result = UpgradeSimulationResult {
        attempts: 0,
        current_level: 0,
    };

    while result.current_level < target_level {
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
