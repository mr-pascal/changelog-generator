use super::file_entry::FileEntry;
use std::collections::HashMap;

pub fn parse_file_name(name: String) -> (String, String) {
    // convention: "TICKTET_NUMBER_SECTION_RND"
    let splitted: Vec<&str> = name.split("_").collect();
    let ticket_reference: &str = splitted.get(0).unwrap(); // TODO2: error handling
    let section: &str = splitted.get(1).unwrap(); // TODO2: error handling
    (String::from(ticket_reference), String::from(section))
}

pub fn create_version_line(version: String, date: String) -> String {
    format!("## [{}] - {}", version, date)
}

pub fn generate_lines(hm: HashMap<String, Vec<FileEntry>>) -> Vec<String> {
    let lines: Vec<String> = hm
        .into_iter()
        .map(|(k, v)| {
            let section_name = k.clone();

            let lines = v
                .into_iter()
                .map(|ve| {
                    // Create text rows from FileEntry
                    return format!("[{}] {}", ve.ticket_reference, ve.content);
                })
                .collect::<Vec<String>>();

            return format!("### {}\n- {}\n", section_name, lines.join("\n- "));
        })
        .collect();
    return lines;
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
        let name = format!("{}_{}_123.md", tr, section);

        // TODO: template string instead of copy&paste
        assert_eq!(parse_file_name(name), (tr, section));

        Ok(())
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
