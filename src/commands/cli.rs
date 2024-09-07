use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

/// Derrik helps you move around data with confidence
#[derive(Parser, Debug)]
#[command(version, bin_name = "derrik", disable_help_subcommand = true)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run(super::run::Cli),
    Read(super::read::Cli),
    Test(super::test::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Run(cli) => cli.run(),
            Commands::Read(cli) => cli.read(),
            Commands::Test(cli) => cli.exec(),
            
        }
    }
}
