use anyhow::Result;
use clap::{Parser, Subcommand};

/// Derrik helps you move around data with confidence
#[derive(Parser, Debug)]
#[command(version, bin_name = "derrik", disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run(super::run::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Run(cli) => cli.run(),
        }
    }
}
