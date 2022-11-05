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

////// ------------------------------------------------

// use crate::entities::{Changelog, ChangelogEntry};
// use std::collections::HashMap;

// #[derive(Clone, Debug)]
// pub struct ChangelogEntry {
//     pub version: String,
//     pub date: String, // TODO: should become a real date!
//     pub sections: HashMap<String, Vec<String>>,
// }

// #[derive(Clone, Debug)]
// pub struct Changelog {
//     pub title: String,
//     pub description: Vec<String>,
//     pub entries: Vec<ChangelogEntry>,
// }
// pub fn convert_changelog_to_string(c: Changelog) -> String {
//     let mut lines: Vec<String> = Vec::new();
//     lines.push(format!("# {}", c.title.clone()));

//     c.description
//         .iter()
//         .for_each(|x| lines.push(String::from(x)));

//     c.entries.iter().for_each(|entry| {
//         lines.push(format!(
//             "## [{}] - {}",
//             entry.version.clone(),
//             entry.date.clone()
//         ));

//         entry.sections.iter().for_each(|(k, v)| {
//             // k -> section name
//             lines.push(format!("### {}", k));
//             v.iter().for_each(|x| {
//                 lines.push(x.clone());
//             })
//         })
//     });
//     let s = lines.join("\n");
//     return s;
// }

// #[derive(Clone)]
// enum CurrentSection {
//     TITLE,
//     DESC,
//     VERSION,
//     MOD,
//     CHANGE,
// }

// fn get_title(s: &str) -> &str {
//     let (_, title_str) = s.split_at(2); // TODO: maybe rather use an indexof?
//     return title_str;
// }

// // "## [1.0.1] - 2022-06-21" -> ("1.0.1", "022-06-21")
// fn parse_version_line(s: &str) -> (String, String) {
//     let s1 = s.replace("## ", "");

//     // TODO: Handling if there is no " - " !
//     let splitted: Vec<&str> = s1.split(" - ").collect();

//     // let (version_dirty, date_dirty) =
//     let version = splitted
//         // TODO: Error handling!
//         .get(0)
//         .expect("Error by getting the version...")
//         .replace(" ", "")
//         .replace("[", "")
//         .replace("]", "");
//     let date_string = splitted
//         // TODO: Error handling!
//         .get(1)
//         .expect("Error by getting the date as string");

//     (version, String::from(*date_string))
// }

// // returns the title
// fn handle_title_line(line: &str) -> &str {
//     get_title(line)
// }

// // Only return the description, no specific handling neede
// fn handle_desc_line(line: &str) -> &str {
//     line
// }

// // Only return the change, no specific handling neede
// fn handle_change_line(line: &str) -> &str {
//     line
// }

// // Returns a new changelog entry
// fn handle_version_line(line: &str) -> ChangelogEntry {
//     // Format: "## [x.y.z] - YYYY-MM-DD"
//     let (version, date_string) = parse_version_line(line);

//     // TODO: Parse "date_string" to real date
//     ChangelogEntry {
//         version: version.clone(),
//         date: date_string,
//         sections: HashMap::new(),
//     }
// }

// // returns the mod title
// fn handle_mod_line(line: &str) -> String {
//     let n = line.replace("### ", "");
//     return n.clone();
// }

// pub fn convert_string_to_changelog(s: String) -> Changelog {
//     let lines = s.lines();

//     let mut title = String::from("");
//     let mut description: Vec<&str> = Vec::new();

//     let mut current_section = CurrentSection::TITLE;
//     let mut entries: Vec<ChangelogEntry> = Vec::new();
//     let mut entries_hash: HashMap<String, ChangelogEntry> = HashMap::new();
//     let mut current_version = String::from("");
//     let mut current_entry_section = String::from("");

//     for line in lines {
//         let is_title_line = line.starts_with("# ");
//         let is_version_line = line.starts_with("## ");
//         let is_mod_line = line.starts_with("### ");

//         // first let's change the state based on the special line
//         if is_title_line {
//             current_section = CurrentSection::TITLE
//         }
//         if is_version_line {
//             current_section = CurrentSection::VERSION
//         }
//         if is_mod_line {
//             current_section = CurrentSection::MOD
//         }

//         match current_section.clone() {
//             CurrentSection::TITLE => {
//                 title = String::from(get_title(line));
//                 // Move to next section
//                 current_section = CurrentSection::DESC;
//             }
//             CurrentSection::DESC => {
//                 description.push(line);
//             }
//             CurrentSection::VERSION => {
//                 let new_entry = handle_version_line(line);
//                 let version = new_entry.version.clone();

//                 entries_hash.insert(version.clone(), new_entry);
//                 current_version = version.clone();

//                 // TODO: only logical, since a MOD has to come afters
//                 // e.g. "### Fixed"
//                 current_section = CurrentSection::MOD;
//             }
//             CurrentSection::MOD => {
//                 // e.g. "### Fixed"
//                 let section_name = handle_mod_line(line);
//                 let v = entries_hash.get_mut(&current_version).unwrap();

//                 // FIXME: duplicated seciton names!!
//                 v.sections
//                     .insert(String::from(section_name.clone()), Vec::new());
//                 current_entry_section = section_name;
//                 // TODO: check if it works or if we have to insert again
//                 // In any case, don#t use mutable but get it, modify it and then set again (functional!)

//                 // entries_hash.insert(String::from(current_version), *val);
//                 current_section = CurrentSection::CHANGE;
//             }
//             CurrentSection::CHANGE => {
//                 let v = entries_hash.get_mut(&current_version).unwrap();
//                 let sec = v.sections.get_mut(&current_entry_section).unwrap();
//                 // FIXME: check if it's enough or if we need to insert again
//                 sec.push(String::from(line.clone()));
//             }
//         }
//         continue;
//     }

//     // FIXME: Implement
//     Changelog {
//         title: title.clone(),
//         description: description.iter().map(|x| String::from(*x)).collect(),
//         // FIXME: order the "entries!"
//         entries: entries_hash.iter().map(|(_, v)| v.clone()).collect(),
//     }
// }
