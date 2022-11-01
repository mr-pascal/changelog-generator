mod entities;
mod parser;
use entities::Changelog;
use parser::{convert_changelog_to_string, convert_string_to_changelog};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::os::unix::prelude::FileExt;
use walkdir::WalkDir;

use crate::entities::ChangelogEntry;

fn read_file(path: String) -> Result<Changelog, Box<dyn Error>> {
    // let error_msg = format!("Couldn't open file {}!", path);
    // TODO: proper error handling
    let changelog_string: String = fs::read_to_string(&path)?.parse()?;

    let ch = convert_string_to_changelog(changelog_string);

    Ok(ch)
}

fn read_file_to_string(path: String) -> Result<String, Box<dyn Error>> {
    let out: String = fs::read_to_string(&path)?.parse()?;
    Ok(out)
}

fn write_string_to_file(path: String, content: String) -> Result<(), Box<dyn Error>> {
    // TODO: Later "write_all_at"

    // let mut file = File::create(path)?;
    // file.write_all(b"Hello, world!")?;

    fs::write(path, content)?;
    Ok(())
}

fn find_changelogs_folder() -> String {
    // TODO: find it by real!
    String::from("changelogs")
}

fn find_changelog_file() -> String {
    // TODO: find it by real!
    String::from("CHANGELOG.md")
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
    // TODO: Format -> <ticket_number>_<action>_<random>.md"

    let mut file_entries: Vec<FileEntry> = vec![];
    println!("find_changelogs: searching in '{}'", folder_path);

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

fn main() {
    println!("Starting...");

    // TODO: paths should be passed via config later

    // 1. Find the changelog file
    // The initial changelog
    let changelog_file_path = String::from("examples/demo1/CHANGELOG.md");

    // Use this file to write the new changelog to TODO -> Will later be the same as the old one
    let changelog_file_path2 = String::from("examples/demo1/CHANGELOG2.md");

    // 2. Find the changelogs to add
    let changelogs_folder_path = String::from("examples/demo1/changelogs");
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
        // .inspect(|x| {
        //     println!("");
        //     println!("{:?}", x);
        // })
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

            return format!("### {}\n- {}", section_name, lines.join("\n- "));
        })
        .collect();

    let combined_lines = lines.join("\n");
    println!("");
    println!("");
    // println!("");
    // println!("{:?}", combined_lines);

    // TODO-NEXT
    // - read actual changelog file

    println!("Try to write into file...");
    let old_changelog_content = read_file_to_string(changelog_file_path.clone()).unwrap(); // TODO2: Error handling
    let split_pattern = "\n## ";
    let mut splitted: Vec<&str> = old_changelog_content.splitn(2, split_pattern).collect();
    println!("{:?}", splitted);

    // Re-Add the pattern that was "splitted" away
    let second_part = splitted.get(1).unwrap();
    let loc = &format!("{}{}", split_pattern, second_part);
    splitted[1] = loc;

    // splitted.insert(1, "\n"); // add a bit of space between entries
    splitted.insert(1, &combined_lines); // Add the actual new lines

    // Combine all the text again and write it
    let final_text = splitted.join("\n");
    write_string_to_file(changelog_file_path2, final_text).unwrap();

    // let f = fs::File::create(changelog_file_path2).expect("Couldn't open file!"); // TODO2: file handler from "read_file_to_string" shoudl be re-used

    // let buffered_lines = combined_lines.as_bytes();
    // let u64_position = u64::try_from(found_last_version_at).expect("conversion failed...");
    // f.write_all_at(buffered_lines, u64_position)
    // .expect("write_all_at failed");

    println!("did it worked?");
    // - find position to insert
    // - insert new point at position

    // TODO-Next
    // - Write version

    // let _ = write_string_to_file("my_new_changelog.md".to_owned(), combined_lines);

    // TODO
    // -- Order by ticket_reference
    // -- Order by section
    //
    // -- Convert to ChangelogEntry
    // -- write changelogentry into CHANGELOG.md

    // Parse the changelogs (create the entries)

    // ----l-------
    // println!("\n");
    // println!("\n");
    // println!("====== Changelog Debug ======");
    // let changelog = read_file(changelog_file.clone()).expect("seems like this program crashed..:/");
    // println!("{:?}", changelog);

    // let stringified = convert_changelog_to_string(changelog);
    // println!("\n");
    // println!("\n");
    // println!("====== Stringified Debug ======");
    // println!("{}", stringified);
    // ----l-------

    // 4. Add the changelogs

    // 5. Cleanup changelog files

    // 6. Set version + Date
}
