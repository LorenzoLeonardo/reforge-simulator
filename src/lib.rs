//! # Reforge Simulator
//!
//! A comprehensive simulation library for mimicking game reforging and item upgrading mechanics.
//! This library provides two main modules:
//!
//! - [`reforging`]: Simulates the reforging system where items are randomly assigned rarity tiers
//! - [`upgrading`]: Simulates the item upgrade system with varying success rates based on item type and level
//!
//! ## Overview
//!
//! The simulator uses weighted probability distributions to model game mechanics realistically.
//! Both reforging and upgrading use the [`ProbabilityDistribution`] enum to represent different outcomes.

use std::error::Error;

use rand::distr::weighted::WeightedIndex;

use crate::{reforging::Rarity, upgrading::Upgrade};

pub mod reforging;
pub mod upgrading;

/// Represents the possible outcomes in probability simulations.
///
/// This enum is used to categorize different types of probabilistic events,
/// enabling a unified approach to handling both reforging and upgrading outcomes.
#[derive(Debug)]
pub enum ProbabilityDistribution {
    /// Outcome from a reforging simulation with an associated rarity level
    Reforging(Rarity),
    /// Outcome from an upgrade simulation with a success/failure status
    Upgrading(Upgrade),
}

/// Constructs a weighted probability distribution from outcome weights.
///
/// # Arguments
///
/// * `weights` - A slice of tuples containing (ProbabilityDistribution, weight) pairs.
///   Weights should sum to 1000 (representing 100% with 0.1% precision).
///
/// # Returns
///
/// A [`Result`] containing a [`WeightedIndex`] for sampling, or an error if construction fails.
///
/// # Example
///
/// ```no_run
/// use reforge_simulator::{ProbabilityDistribution, reforging::Rarity, build_distribution};
///
/// let weights = vec![
///     (ProbabilityDistribution::Reforging(Rarity::Uncommon), 530),
///     (ProbabilityDistribution::Reforging(Rarity::Rare), 300),
/// ];
/// let dist = build_distribution(&weights).unwrap();
/// ```
pub fn build_distribution(
    weights: &[(ProbabilityDistribution, u64)],
) -> Result<WeightedIndex<u64>, Box<dyn Error>> {
    let weights: Vec<u64> = weights.iter().map(|(_, w)| *w).collect();
    Ok(WeightedIndex::new(weights)?)
}
