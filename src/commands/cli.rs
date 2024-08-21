use anyhow::Result;
use clap::Parser;

/// Derrik helps you move around data with confidence
#[derive(Parser, Debug)]
#[command(version, bin_name = "derrik", disable_help_subcommand = true)]
pub struct Cli {}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        println!("Hello, World!");

        Ok(())
    }
}
