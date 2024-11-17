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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", content).unwrap();
        file
    }

    #[test]
    fn test_filter_contains() -> Result<()> {
        // Create a temporary input file
        let json_content = r#"{"name": "John Doe", "age": 30}"#;
        let input_file = create_test_file(json_content);
        let output_file = NamedTempFile::new()?;

        let cli = Cli {
            input_files: vec![input_file.path().to_path_buf()],
            _where: vec!["name".to_string()],
            operator: Some(Operator::Contains),
            what: "John".to_string(),
            output: Some(output_file.path().to_path_buf()),
        };

        cli.filter()?;

        let output_content = fs::read_to_string(output_file.path())?;
        assert!(output_content.contains(json_content));

        let cli = Cli {
            input_files: vec![input_file.path().to_path_buf()],
            _where: vec!["name".to_string()],
            operator: Some(Operator::Contains),
            what: "john".to_string(),
            output: Some(output_file.path().to_path_buf()),
        };

        cli.filter()?;

        let output_content = fs::read_to_string(output_file.path())?;
        assert!(output_content.is_empty());
        Ok(())
    }

    #[test]
    fn test_filter_icontains() -> Result<()> {
        let json_content = r#"{"name": "John Doe", "age": 30}"#;
        let input_file = create_test_file(json_content);
        let output_file = NamedTempFile::new()?;

        let cli = Cli {
            input_files: vec![input_file.path().to_path_buf()],
            _where: vec!["name".to_string()],
            operator: Some(Operator::Icontains),
            what: "john".to_string(), // Note: lowercase
            output: Some(output_file.path().to_path_buf()),
        };

        cli.filter()?;

        let output_content = fs::read_to_string(output_file.path())?;
        assert!(output_content.contains(json_content));
        Ok(())
    }

    #[test]
    fn test_filter_no_match() -> Result<()> {
        let json_content = r#"{"name": "John Doe", "age": 30}"#;
        let input_file = create_test_file(json_content);
        let output_file = NamedTempFile::new()?;

        let cli = Cli {
            input_files: vec![input_file.path().to_path_buf()],
            _where: vec!["name".to_string()],
            operator: Some(Operator::Contains),
            what: "Alice".to_string(),
            output: Some(output_file.path().to_path_buf()),
        };

        cli.filter()?;

        let output_content = fs::read_to_string(output_file.path())?;
        assert!(output_content.is_empty());
        Ok(())
    }

    #[test]
    fn test_filter_multiple_fields() -> Result<()> {
        let json_content = r#"{"name": "John Doe", "description": "Software Engineer"}"#;
        let input_file = create_test_file(json_content);
        let output_file = NamedTempFile::new()?;

        let cli = Cli {
            input_files: vec![input_file.path().to_path_buf()],
            _where: vec!["name".to_string(), "description".to_string()],
            operator: Some(Operator::Contains),
            what: "Engineer".to_string(),
            output: Some(output_file.path().to_path_buf()),
        };

        cli.filter()?;

        let output_content = fs::read_to_string(output_file.path())?;
        assert!(output_content.contains(json_content));
        Ok(())
    }

    #[test]
    fn test_filter_multiple_files() -> Result<()> {
        let json_content1 = r#"{"name": "John Doe", "age": 30}"#;
        let json_content2 = r#"{"name": "Jane Doe", "age": 25}"#;
        let input_file1 = create_test_file(json_content1);
        let input_file2 = create_test_file(json_content2);
        let output_file = NamedTempFile::new()?;

        let cli = Cli {
            input_files: vec![
                input_file1.path().to_path_buf(),
                input_file2.path().to_path_buf(),
            ],
            _where: vec!["name".to_string()],
            operator: Some(Operator::Contains),
            what: "Doe".to_string(),
            output: Some(output_file.path().to_path_buf()),
        };

        cli.filter()?;

        let output_content = fs::read_to_string(output_file.path())?;
        assert!(output_content.contains(json_content1));
        assert!(output_content.contains(json_content2));
        Ok(())
    }

    #[test]
    fn test_filter_invalid_json() -> Result<()> {
        let invalid_json = r#"{"name": "John Doe", age: 30}"#; // Missing quotes around age
        let input_file = create_test_file(invalid_json);
        let output_file = NamedTempFile::new()?;

        let cli = Cli {
            input_files: vec![input_file.path().to_path_buf()],
            _where: vec!["name".to_string()],
            operator: Some(Operator::Contains),
            what: "John".to_string(),
            output: Some(output_file.path().to_path_buf()),
        };

        cli.filter()?;

        let output_content = fs::read_to_string(output_file.path())?;
        assert!(output_content.is_empty());
        Ok(())
    }
}
