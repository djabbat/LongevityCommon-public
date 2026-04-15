/// Ze Runner — CLI entry point for Ze simulators
///
/// Usage:
///   ze-runner thermo [--molecules N] [--steps T] [--demon] [--cold-start] [--seed K]
///   ze-runner quantum [--dim D] [--steps T] [--states S] [--seed K]
///   ze-runner repro   [--tau0 T] [--chains N] [--dim D] [--seed K]
///
/// Output: JSON to stdout (for Phoenix LiveView / plotting)

use clap::{Parser, Subcommand};
use ze_core::{thermo, quantum, reproduction};

#[derive(Parser)]
#[command(name = "ze-runner", about = "Ze Vector Theory simulator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run Ze Thermodynamic simulator (Level 2)
    Thermo {
        #[arg(long, default_value = "100")]
        molecules: usize,
        #[arg(long, default_value = "500")]
        steps: usize,
        #[arg(long, default_value = "false")]
        demon: bool,
        /// Cold start: all velocities initialised at v=0 (required for Second-Law demo).
        /// At equilibrium S_Boltzmann is already maximal; cold start ensures both
        /// S_Ze and S_Boltzmann start at 0 and increase, giving positive Spearman correlation.
        #[arg(long, default_value = "true")]
        cold_start: bool,
        #[arg(long, default_value = "42")]
        seed: u64,
    },
    /// Run Ze Quantum simulator (Level 3)
    Quantum {
        #[arg(long, default_value = "4")]
        dim: usize,
        #[arg(long, default_value = "2000")]
        steps: usize,
        #[arg(long, default_value = "50")]
        states: usize,
        #[arg(long, default_value = "42")]
        seed: u64,
    },
    /// Run Ze Reproduction simulator (Level 4) — Axiom Z4 + double-slit
    Repro {
        #[arg(long, default_value = "200")]
        tau0: i64,
        #[arg(long, default_value = "500")]
        chains: usize,
        #[arg(long, default_value = "4")]
        dim: usize,
        #[arg(long, default_value = "42")]
        seed: u64,
    },
}

fn main() {
    let cli = Cli::parse();
    let json = match cli.command {
        Commands::Thermo { molecules, steps, demon, cold_start, seed } => {
            let result = thermo::run_thermo(molecules, steps, demon, cold_start, seed);
            serde_json::to_string(&result).unwrap()
        }
        Commands::Quantum { dim, steps, states, seed } => {
            let result = quantum::run_quantum(dim, steps, states, seed);
            serde_json::to_string(&result).unwrap()
        }
        Commands::Repro { tau0, chains, dim, seed } => {
            let result = reproduction::run_reproduction(tau0, chains, dim, seed);
            serde_json::to_string(&result).unwrap()
        }
    };
    println!("{}", json);
}
