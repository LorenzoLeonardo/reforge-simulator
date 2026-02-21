mod reforging;
mod upgrading;

use std::error::Error;

use rand::distr::weighted::WeightedIndex;

use crate::{
    reforging::Rarity,
    upgrading::{ItemType, Upgrade},
};

#[derive(Debug)]
pub(crate) enum ProbabilityDistribution {
    Reforging(Rarity),
    Upgrading(Upgrade),
}

pub(crate) fn build_distribution(
    weights: &[(ProbabilityDistribution, u64)],
) -> Result<WeightedIndex<u64>, Box<dyn Error>> {
    let weights: Vec<u64> = weights.iter().map(|(_, w)| *w).collect();
    Ok(WeightedIndex::new(weights)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let limit = args
        .get(1) // safely get index 1
        .and_then(|s| s.parse::<u64>().ok()) // try parse
        .unwrap_or(1); // default to 1 if missing or invalid

    let result = reforging::run_reforge_simulation(limit, 0)?;

    println!("=== Simulation Result ===");
    println!("Attempts until Ancient: {}", result.attempts);
    println!();
    println!("Uncommon:  {}", result.uncommon);
    println!("Rare:      {}", result.rare);
    println!("Epic:      {}", result.epic);
    println!("Legendary: {}", result.legendary);
    println!("Ancient:   {}", result.ancient);

    println!(
        "\n--- Running Upgrade Simulation for {:?} Items (Max Level {}) ---\n",
        ItemType::Normal,
        ItemType::Normal.level_cap()
    );
    let upgrade_result = upgrading::run_upgrade_simulation(&ItemType::Normal)?;
    println!(
        "\n=== {:?} ItemUpgrade Simulation Result ===",
        ItemType::Normal
    );
    println!("Final Item Level: {}", upgrade_result.current_level);
    println!("Total Upgrade Attempts: {}", upgrade_result.attempts);
    println!();

    println!(
        "\n--- Running Upgrade Simulation for {:?} Items (Max Level {}) ---\n",
        ItemType::Accessories,
        ItemType::Accessories.level_cap()
    );
    let upgrade_result = upgrading::run_upgrade_simulation(&ItemType::Accessories)?;
    println!(
        "\n=== {:?} ItemUpgrade Simulation Result ===",
        ItemType::Accessories
    );
    println!("Final Item Level: {}", upgrade_result.current_level);
    println!("Total Upgrade Attempts: {}", upgrade_result.attempts);
    println!();

    println!(
        "\n--- Running Upgrade Simulation for {:?} Items (Max Level {}) ---\n",
        ItemType::Baruna,
        ItemType::Baruna.level_cap()
    );
    let upgrade_result = upgrading::run_upgrade_simulation(&ItemType::Baruna)?;
    println!(
        "\n=== {:?} ItemUpgrade Simulation Result ===",
        ItemType::Baruna
    );
    println!("Final Item Level: {}", upgrade_result.current_level);
    println!("Total Upgrade Attempts: {}", upgrade_result.attempts);
    println!();

    Ok(())
}
