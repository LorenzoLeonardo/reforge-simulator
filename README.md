# Reforge Simulator

A small Rust library and CLI tooling to simulate game-style "reforging" (rarity rolls) and item upgrading mechanics.

**Features**
- Reforging simulation with rarity tiers and a luck modifier
- Upgrade simulation for multiple item types with level-specific success rates
- Two CLI binaries: `reforge` and `upgrade` for quick experiments

**Quick Start**

Build the project:

```bash
cargo build --release
```

Run the reforging simulator (example, 100 attempts):

```bash
cargo run --bin reforge -- 100
```

Run the upgrade simulator (example, 500 attempts for `--normal` items):

```bash
cargo run --bin upgrade -- 500 --normal
```

**Binaries & Usage**
- `reforge`: Simulates reforging until an Ancient or the provided limit.
  - Usage: `cargo run --bin reforge -- [LIMIT]`
- `upgrade`: Simulates upgrading an item type up to its cap or the provided limit.
  - Usage: `cargo run --bin upgrade -- [LIMIT] [TYPE]`
  - `TYPE` must be one of: `--normal`, `--accessories`, `--baruna`

**Library API** (for programmatic use)
- `reforge_simulator::reforging` — reforging module
  - `run_reforge_simulation(limit: u64, luck: u64) -> Result<ReforgeSimulationResult, _>`
- `reforge_simulator::upgrading` — upgrading module
  - `run_upgrade_simulation(limit: u64, item_type: &ItemType) -> Result<UpgradeSimulationResult, _>`

**Development**
- Format: `cargo fmt`
- Lints: `cargo clippy --all -- -D warnings`
- Run: `cargo run --bin reforge` or `cargo run --bin upgrade`

**Contributing**
PRs welcome. Please run `cargo fmt` and `cargo clippy` before opening a PR.

**License**
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
