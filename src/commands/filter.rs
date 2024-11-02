use anyhow::Result;
use clap::{Args, ValueEnum};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Operator {
    Contains,
    Icontains,
}

/// Filter file content
///
/// All arguments are passed through unless --help is first
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    input_files: Vec<PathBuf>,

    /// The name of the field that will be use for filtering.
    #[arg(long="where", required = true, value_delimiter = ' ', num_args = 1..)]
    _where: Vec<String>,

    /// Operators: "contains" or "icontains" (insensitive-case).
    #[arg(long, value_enum)]
    operator: Option<Operator>,

    /// Keyword to search for.
    #[arg(long, required = true)]
    what: String,

    /// Path to the output file.
    #[arg(long)]
    output: Option<PathBuf>,
}

impl Cli {
    pub fn filter(&self) -> Result<()> {
        use serde_json::Value;

        let operator = &self.operator.unwrap_or(Operator::Contains);

        // Determine where to write output
        let output: Box<dyn Write> = if self.output.is_none() {
            // Use stdout if no output destination was passed
            Box::new(std::io::stdout().lock())
        } else {
            // Open the destination file for writing
            Box::new(File::create(self.output.as_ref().unwrap().as_path())?)
        };
        let mut output = std::io::BufWriter::new(output);

        for f in &self.input_files {
            let input_file = File::open(f)?;
            let reader = BufReader::new(input_file);

            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        if let Ok(json_value) = serde_json::from_str::<Value>(&line) {
                            let mut matched = false;
                            for field in &self._where {
                                if let Some(field_value) = json_value.get(field) {
                                    let mut field_str = field_value.to_string();
                                    if *operator == Operator::Icontains {
                                        field_str = field_str.to_lowercase()
                                    }

                                    if field_str.contains(&self.what) {
                                        matched = true;
                                        break;
                                    }
                                }
                            }
                            if matched {
                                writeln!(output, "{}", line)?;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading line: {}", e);
                    }
                }
            }
        }
        output.flush()?;

        Ok(())
    }
}
