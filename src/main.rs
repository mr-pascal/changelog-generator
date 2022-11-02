mod entities;
mod parser;
use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use walkdir::WalkDir;

fn read_file_to_string(path: String) -> Result<String, Box<dyn Error>> {
    let out: String = fs::read_to_string(&path)?.parse()?;
    Ok(out)
}

fn write_string_to_file(path: String, content: String) -> Result<(), Box<dyn Error>> {
    fs::write(path, content)?;
    Ok(())
}

#[derive(Clone, Debug)]
struct FileEntry {
    file_name: String,
    path: String,
    content: String,
    ticket_reference: String,
    section: String,
}

fn find_changelogs(folder_path: String) -> Vec<FileEntry> {
    // Format -> <ticket_number>_<action>_<random>.md"

    let mut file_entries: Vec<FileEntry> = vec![];
    // println!("find_changelogs: searching in '{}'", folder_path);

    WalkDir::new(folder_path)
        .into_iter()
        // .filter_entry(|e| !is_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| {
            if x.depth() != 1 {
                // Ignore everything which isn't a direct
                // descendant of 'folder_path'
                return;
            }

            file_entries.push(FileEntry {
                file_name: x.file_name().to_str().unwrap().to_owned(), // TODO2: Maybe some error handling later
                path: x.path().to_str().unwrap().to_owned(), // TODO2: maybe some error handling later
                content: String::from(""),
                ticket_reference: String::from(""),
                section: String::from(""),
            });
        });
    return file_entries;
}

fn parse_file_name(name: String) -> (String, String) {
    // convention: "TICKTET_NUMBER_SECTION_RND"
    let splitted: Vec<&str> = name.split("_").collect();
    let ticket_reference: &str = splitted.get(0).unwrap(); // TODO2: error handling
    let section: &str = splitted.get(1).unwrap(); // TODO2: error handling
    (String::from(ticket_reference), String::from(section))
}

fn group_by_section(entries: Vec<FileEntry>) -> HashMap<String, Vec<FileEntry>> {
    let mut hm: HashMap<String, Vec<FileEntry>> = HashMap::new();
    entries.into_iter().for_each(|entry| {
        let section: String = entry.section.clone();
        if !hm.contains_key(&section) {
            // nothing in there yet
            hm.insert(section.clone(), vec![]);
        }
        // get mut so we don't have to re-insert it
        let v = hm.get_mut(&section).unwrap(); // TODO2: error handling
        v.push(entry);
    });
    return hm;
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_group_by_section() -> Result<(), String> {
        let a = FileEntry {
            file_name: "a".to_owned(),
            path: "a".to_owned(),
            content: "a".to_owned(),
            section: "a".to_owned(),
            ticket_reference: "a".to_owned(),
        };
        let b = FileEntry {
            file_name: "b".to_owned(),
            path: "b".to_owned(),
            content: "b".to_owned(),
            section: "a".to_owned(),
            ticket_reference: "b".to_owned(),
        };
        let c = FileEntry {
            file_name: "c".to_owned(),
            path: "c".to_owned(),
            content: "c".to_owned(),
            section: "c".to_owned(),
            ticket_reference: "c".to_owned(),
        };

        let entries = vec![a.clone(), b.clone(), c.clone()];
        let expected: HashMap<String, Vec<FileEntry>> = HashMap::from([
            (a.section.clone(), vec![a, b]),
            (c.section.clone(), vec![c]),
        ]);
        let grouped = group_by_section(entries);
        grouped.keys().for_each(|g| {
            assert_eq!(expected.contains_key(g), true);
        });

        for (k, v) in grouped {
            // Only checking "roughly"
            // no in-depth checks for actual content
            assert_eq!(expected.contains_key(&k), true);
            assert_eq!(expected.get(&k).unwrap().len(), v.len());
        }

        Ok(())
    }
}

fn create_version_line(version: String, date: String) -> String {
    format!("## [{}] - {}", version, date)
}

// Comand line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the destination changelog file
    #[clap(short, long, value_parser, default_value_t = String::from("CHANGELOG.md"))]
    changelog_path: String,

    /// Path to the folder containing the change logs
    #[clap(short, long, value_parser, default_value_t = String::from("changelogs"))]
    folder_path: String,

    /// New version to set
    #[clap(short, long, value_parser)]
    new_version: String,

    /// Delete change log files after merging?
    #[clap(short, long, value_parser, default_value_t = false)]
    delete_changelogs: bool,
    // TODO3: add optional date
}

fn main() {
    // Parse aaaaaaaaa
    let args = Args::parse();
    println!("Arguments:");
    println!("{:?}", args);

    // Extract arguments
    let changelog_file_path = args.changelog_path;
    let changelogs_folder_path = args.folder_path;
    let new_version = args.new_version;
    let delete_changelogs = args.delete_changelogs;

    if delete_changelogs {
        println!("The '-d' and '--delete_changelogs' arguments are not yet implemented!");
        std::process::exit(1);
    }

    // Find all the new entries
    let file_entries = find_changelogs(changelogs_folder_path);

    // 3. Read the files
    // TODO2: for "practice sake" use tokio spawn to read the files concurrently?
    //        Could also add some benchmarking if it improves performance
    let result: Vec<FileEntry> = file_entries
        .into_iter()
        .map(|entry| {
            // TODO2: imperative approach is most likely faster
            let content = read_file_to_string(entry.path.clone()).unwrap(); //TODO2: error handling
            let (ticket_reference, section) = parse_file_name(entry.file_name.clone());

            // TODO2: create FileEntry constructur, to prevent manual copy
            return FileEntry {
                path: entry.path,
                file_name: entry.file_name,
                content,
                section,
                ticket_reference,
            };
        })
        .collect();

    let grouped = group_by_section(result);

    let lines: Vec<String> = grouped
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

    let combined_lines = lines.join("\n");
    println!("");
    println!("");

    // TODO: Those have to be dynamically!
    let new_date = String::from("2022-11-01");
    let version_line = create_version_line(new_version, new_date);

    // println!("Try to write into file...");
    let old_changelog_content = read_file_to_string(changelog_file_path.clone()).unwrap(); // TODO2: Error handling
    let split_pattern = "\n## ";
    let mut splitted: Vec<&str> = old_changelog_content.splitn(2, split_pattern).collect();
    // println!("{:?}", splitted);

    // Re-Add the pattern that was "splitted" away
    let second_part = splitted.get(1).unwrap();
    let loc = &format!("{}{}", split_pattern, second_part);
    splitted[1] = loc;

    // splitted.insert(1, "\n"); // add a bit of space between entries
    splitted.insert(1, &combined_lines); // Add the actual new lines
    splitted.insert(1, &version_line); // Add version and date

    // Combine all the text again and write it
    let final_text = splitted.join("\n");
    write_string_to_file(changelog_file_path, final_text).expect("Should write to the file...");

    // 5. Cleanup changelog files
}
