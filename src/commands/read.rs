use anyhow::Result;
use clap::Args;
use std::process::exit;
use std::fs;


/// Print file content
///
/// All arguments are passed through unless --help is first
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

impl Cli {
    pub fn read(&self) -> Result<()> {
        // Get the file.txt path
        // Check if the file.txt exists
        // Print the content
        let contents = fs::read_to_string(&self.args[0])
            .expect("The file path doesn't exist");
        println!("With text:\n{contents}");

        exit(0);
    }
}
