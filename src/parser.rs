use super::file_entry::FileEntry;
use colored::Colorize;
use std::collections::HashMap;
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
    fn new(file_name: String) -> ParseFileNameError {
        ParseFileNameError { file_name }
    }
}

pub fn parse_file_name(name: String) -> Result<(String, String), ParseFileNameError> {
    let splitted: Vec<&str> = name.split('_').collect();

    let ticket_reference: &str = splitted
        .first()
        .ok_or_else(|| ParseFileNameError::new(name.clone()))?;
    // TODO2: Check if "ok_or" or "ok_or_else" is better for throwning new errors

    let section: &str = splitted
        .get(1)
        .ok_or_else(|| ParseFileNameError::new(name.clone()))?;

    Ok((String::from(ticket_reference), String::from(section)))
}

pub fn create_version_line(version: String, date: String) -> String {
    format!("## [{}] - {}", version, date)
}

pub fn generate_lines(hm: HashMap<String, Vec<FileEntry>>) -> Vec<String> {
    let lines: Vec<String> = hm
        .into_iter()
        .map(|(k, v)| {
            let section_name = k;

            let lines = v
                .into_iter()
                .map(|ve| {
                    // Create text rows from FileEntry
                    format!("[{}] {}", ve.ticket_reference, ve.content)
                })
                .collect::<Vec<String>>();

            format!("### {}\n- {}\n", section_name, lines.join("\n- "))
        })
        .collect();
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_lines() -> Result<(), String> {
        // TODO: Implement test!
        Ok(())
    }

    #[test]
    fn test_parse_file_name() -> Result<(), String> {
        let tr = String::from("AI-123");
        let section = String::from("changed");
        let input = format!("{}_{}_123.md", tr, section);
        let expected = (tr, section);
        let output = parse_file_name(input).unwrap();
        assert_eq!(output, expected);
        Ok(())

        // TODO: check also the Error case!
    }

    #[test]
    fn test_create_version_line() -> Result<(), String> {
        assert_eq!(
            create_version_line("1.2.3".to_owned(), "2022-11-01".to_owned()),
            "## [1.2.3] - 2022-11-01"
        );
        Ok(())
    }
}
