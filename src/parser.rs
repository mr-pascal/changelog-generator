use super::parse_file_name_error::ParseFileNameError;

use super::file_entry::FileEntry;
use std::collections::HashMap;

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
    // TODO3: instead of using a Vec<FileEntry> use something more generic
    // so this method isn't depending on FileEntry
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
        let mut input: HashMap<String, Vec<FileEntry>> = HashMap::new();
        input.insert(
            "Added".to_owned(),
            vec![
                FileEntry {
                    file_name: "WAYNE".to_owned(),
                    path: "WAYNE".to_owned(),
                    content: "This is my\nmultiline content".to_owned(),
                    ticket_reference: "MULTI-123".to_owned(),
                    section: "WAYNE".to_owned(),
                },
                FileEntry {
                    file_name: "WAYNE".to_owned(),
                    path: "WAYNE".to_owned(),
                    content: "My second content".to_owned(),
                    ticket_reference: "PL-123".to_owned(),
                    section: "WAYNE".to_owned(),
                },
            ],
        );
        input.insert(
            "Changed".to_owned(),
            vec![
                FileEntry {
                    file_name: "WAYNE".to_owned(),
                    path: "WAYNE".to_owned(),
                    content: "Changed some\nthings".to_owned(),
                    ticket_reference: "CH-123".to_owned(),
                    section: "WAYNE".to_owned(),
                },
                FileEntry {
                    file_name: "WAYNE".to_owned(),
                    path: "WAYNE".to_owned(),
                    content: "My second change".to_owned(),
                    ticket_reference: "CH-256".to_owned(),
                    section: "WAYNE".to_owned(),
                },
            ],
        );

        let output = generate_lines(input);
        let expected: Vec<String> = vec![
            "### Added\n- [MULTI-123] This is my\nmultiline content\n- [PL-123] My second content\n"
                .to_owned(),
            "### Changed\n- [CH-123] Changed some\nthings\n- [CH-256] My second change\n"
                .to_owned(),
        ];
        assert_eq!(output, expected);
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
