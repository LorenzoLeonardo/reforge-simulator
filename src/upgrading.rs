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

#[derive(Debug)]
pub(crate) enum Upgrade {
    Success,
    Failure,
}

pub(crate) struct UpgradeSimulationResult {
    pub attempts: u64,
    pub current_level: u64,
}

fn get_upgrade_success_rate(current_level: u64) -> u64 {
    if current_level >= 20 {
        panic!("Cannot upgrade beyond level 20");
    }
    ACCESSORIES_UPGRADE_RATES[current_level as usize]
}

fn init_upgrade_weights(current_level: u64) -> Vec<(ProbabilityDistribution, u64)> {
    let success_rate = get_upgrade_success_rate(current_level);
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
    target_level: u64,
) -> Result<UpgradeSimulationResult, Box<dyn Error>> {
    let mut rng = rand::rng();

    let mut result = UpgradeSimulationResult {
        attempts: 0,
        current_level: 0,
    };

    while result.current_level < target_level {
        let weights = init_upgrade_weights(result.current_level);
        let dist = build_distribution(&weights)?;
        result.attempts += 1;

        print!(
            "Attempt {} (Level {}) . . .",
            result.attempts, result.current_level
        );
        std::io::stdout().flush()?;
        std::thread::sleep(std::time::Duration::from_millis(500)); // Simulate time delay for each attempt
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
