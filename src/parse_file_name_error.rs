use colored::Colorize;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseFileNameError {
    file_name: String,
}

impl fmt::Display for ParseFileNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: '{}' doesn't match the pattern! (<TICKET_NUMBER>_<SECTION>_...)",
            "ERROR".red(),
            self.file_name
        )
    }
}

impl Error for ParseFileNameError {}

impl From<String> for ParseFileNameError {
    fn from(file_name: String) -> Self {
        ParseFileNameError { file_name }
    }
}

impl ParseFileNameError {
    pub fn new(file_name: String) -> ParseFileNameError {
        ParseFileNameError { file_name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ParseFileNameError_new() -> Result<(), String> {
        let output = ParseFileNameError::new("my_file_name.txt".to_owned());
        let expected = ParseFileNameError {
            file_name: "my_file_name.txt".to_owned(),
        };
        assert_eq!(output.file_name, expected.file_name);

        Ok(())
    }
}
