mod entities;
mod parser;

use entities::Changelog;
use parser::{convert_changelog_to_string, convert_string_to_changelog};
use std::error::Error;
use std::fs;
use walkdir::{DirEntry, WalkDir};

fn read_file(path: String) -> Result<Changelog, Box<dyn Error>> {
    // let error_msg = format!("Couldn't open file {}!", path);
    // TODO: proper error handling
    let changelog_string: String = fs::read_to_string(&path)?.parse()?;

    let ch = convert_string_to_changelog(changelog_string);

    Ok(ch)
}

fn find_changelogs_folder() -> String {
    // TODO: find it by real!
    String::from("changelogs")
}

fn find_changelog_file() -> String {
    // TODO: find it by real!
    String::from("CHANGELOG.md")
}

fn find_changelogs(folder_path: String) -> Vec<String> {
    // TODO: Format -> <ticket_number>_<action>_<random>.md"

    let mut file_paths: Vec<String> = Vec::new();
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
            // println!("Visible: {}", path);
            file_paths.push(String::from(x.path().to_str().expect("will work")));
        });
    return file_paths;
}

fn main() {
    println!("Starting...");

    // TODO: paths should be passed via config later

    // 1. Find the changelog file
    let changelog_file = String::from("examples/demo1/CHANGELOG.md");

    // 2. Find the changelogs to add
    let changelogs_folder_path = String::from("examples/demo1/changelogs");
    let file_paths = find_changelogs(changelogs_folder_path);
    println!("Found files: {:?}", file_paths);

    // TODO
    // -- Read the files
    // -- get filename from pile path
    // -- hashmap <filename, file string content>
    // -- Convert file string content to ChangelogEntry
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
