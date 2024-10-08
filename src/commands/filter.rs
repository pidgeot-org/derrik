use anyhow::Result;
use clap::{Args, ValueEnum};
use std::process::exit;
use std::fs;
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Operator {
    Contains,
    Icontains,
}

/// Print file content
// 3 slashes is a doc comment
///
/// All arguments are passed through unless --help is first
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    input_files: Vec<PathBuf>,

    /// The name of the field that will be use for filtering.
    #[arg(long)]
    text_key: Option<String>,

    /// Operators: contains or icontains (insensitive-case).
    #[arg(long, value_enum)]
    operator: Option<Operator>,

    /// Keywords to match, space separated.
    #[arg(long)]
    matches: Option<Vec<String>>,
    // ['a','b','c']

    /// Path to file containing keywords to match.
    #[arg(long)]
    match_file: Option<PathBuf>,

    /// Keywords to match, space separated.
    #[arg(long)]
    match_key: Option<String>,

    /// Path to the output.
    #[arg(long)]
    output: Option<PathBuf>,

}

impl Cli {
    pub fn filter(&self) -> Result<()> {
        // Get the file.txt path
        // Check if the file.txt exists
        // Print the content
        let _contents = fs::read_to_string(&self.input_files[0])
            .expect("The file path doesn't exist");
        // println!("With text:\n{contents}");
        if let Some(s) = &self.text_key {
            println!("--text-key: {}", s);
        }
        let error_message = String::from("Error");
        //let text_key: &String = &self.text_key.as_ref().unwrap_or(&error_message);
        let text_key: &String = &self.text_key.as_ref().unwrap_or_else(|| { return &error_message; });
        println!("--text-key: {text_key}");
        exit(0);
    }
}
