use anyhow::Result;
use clap::{Args, ValueEnum};
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
        use serde_json::Value;
        let contents =
            fs::read_to_string(&self.input_files[0]).expect("The file path doesn't exist");

        let text_key = self.text_key.as_ref().expect("--text-key is required");
        let match_key = self.match_key.as_ref().expect("--match-key is required");

        for line in contents.lines() {
            if let Ok(json_value) = serde_json::from_str::<Value>(line) {
                if let Some(field_value) = json_value.get(text_key) {
                    let field_str = match field_value {
                        Value::String(s) => s.to_string(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null => "null".to_string(),
                        _ => continue,
                    };

                    if field_str == *match_key {
                        println!("{}", line);
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_file_with_content(content: &str) -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", content).unwrap();
        temp_file
    }

    #[test]
    fn test_filter_exact_match() {
        let json_content = r#"{"name": "John", "age": 30}
{"name": "Alice", "age": 25}
{"name": "Bob", "age": 30}"#;

        let temp_file = create_temp_file_with_content(json_content);

        let cli = Cli {
            input_files: vec![temp_file.path().to_path_buf()],
            text_key: Some("age".to_string()),
            operator: None,
            matches: None,
            match_file: None,
            match_key: Some("30".to_string()),
            output: None,
        };

        let result = cli.filter();
        assert!(result.is_ok());
    }

    #[test]
    fn test_filter_string_match() {
        let json_content = r#"{"name": "John", "country": "USA"}
{"name": "Alice", "country": "Canada"}
{"name": "Bob", "country": "USA"}"#;

        let temp_file = create_temp_file_with_content(json_content);

        let cli = Cli {
            input_files: vec![temp_file.path().to_path_buf()],
            text_key: Some("country".to_string()),
            operator: None,
            matches: None,
            match_file: None,
            match_key: Some("USA".to_string()),
            output: None,
        };

        let result = cli.filter();
        assert!(result.is_ok());
    }

    #[test]
    fn test_filter_boolean_match() {
        let json_content = r#"{"name": "John", "active": true}
{"name": "Alice", "active": false}
{"name": "Bob", "active": true}"#;

        let temp_file = create_temp_file_with_content(json_content);

        let cli = Cli {
            input_files: vec![temp_file.path().to_path_buf()],
            text_key: Some("active".to_string()),
            operator: None,
            matches: None,
            match_file: None,
            match_key: Some("true".to_string()),
            output: None,
        };

        let result = cli.filter();
        assert!(result.is_ok());
    }

    #[test]
    fn test_filter_null_match() {
        let json_content = r#"{"name": "John", "email": null}
{"name": "Alice", "email": "alice@example.com"}
{"name": "Bob", "email": null}"#;

        let temp_file = create_temp_file_with_content(json_content);

        let cli = Cli {
            input_files: vec![temp_file.path().to_path_buf()],
            text_key: Some("email".to_string()),
            operator: None,
            matches: None,
            match_file: None,
            match_key: Some("null".to_string()),
            output: None,
        };

        let result = cli.filter();
        assert!(result.is_ok());
    }

    #[test]
    #[should_panic(expected = "--text-key is required")]
    fn test_missing_text_key() {
        let json_content = r#"{"name": "John", "age": 30}"#;
        let temp_file = create_temp_file_with_content(json_content);

        let cli = Cli {
            input_files: vec![temp_file.path().to_path_buf()],
            text_key: None,
            operator: None,
            matches: None,
            match_file: None,
            match_key: Some("30".to_string()),
            output: None,
        };

        let _ = cli.filter();
    }

    #[test]
    #[should_panic(expected = "--match-key is required")]
    fn test_missing_match_key() {
        let json_content = r#"{"name": "John", "age": 30}"#;
        let temp_file = create_temp_file_with_content(json_content);

        let cli = Cli {
            input_files: vec![temp_file.path().to_path_buf()],
            text_key: Some("age".to_string()),
            operator: None,
            matches: None,
            match_file: None,
            match_key: None,
            output: None,
        };

        let _ = cli.filter();
    }

    #[test]
    fn test_invalid_json() {
        let json_content = r#"{"name": "John", "age": 30}
invalid json
{"name": "Bob", "age": 30}"#;

        let temp_file = create_temp_file_with_content(json_content);

        let cli = Cli {
            input_files: vec![temp_file.path().to_path_buf()],
            text_key: Some("age".to_string()),
            operator: None,
            matches: None,
            match_file: None,
            match_key: Some("30".to_string()),
            output: None,
        };

        let result = cli.filter();
        assert!(result.is_ok());
    }
}
