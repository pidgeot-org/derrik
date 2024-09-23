use anyhow::Result;
use clap::{Args, ValueEnum};
use std::process::exit;
use std::fs;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Operator {
    Contains,
    Icontains,
}

/// Print file content
///
/// All arguments are passed through unless --help is first
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    input_files: Vec<String>,

    /// The name of the field that will be use for filtering.
    #[arg(long)]
    text_key: Option<String>,

    #[arg(long, value_enum)]
    operator: Option<Operator>,

    // #[arg(long)]
    // matches: Vec<String>,
}

impl Cli {
    pub fn filter(&self) -> Result<()> {
        // Get the file.txt path
        // Check if the file.txt exists
        // Print the content
        let contents = fs::read_to_string(&self.input_files[0])
            .expect("The file path doesn't exist");
        println!("With text:\n{contents}");

        exit(0);
    }
}
