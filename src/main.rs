mod file_entry;
mod filesystem;
mod parser;
mod utils;
mod walker;
use clap::Parser;
use file_entry::{group_by_section, FileEntry};
use filesystem::{read_file_to_string, write_string_to_file};
use parser::{create_version_line, generate_lines};
use utils::combine_lines;
use walker::find_and_convert_changelogs;

use crate::filesystem::remove_files;

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

    /// The date string to set for the new version
    #[clap(long, value_parser)]
    date: String,

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
    let date = args.date;
    let delete_changelogs = args.delete_changelogs;

    // Find all the new entries
    let result = find_and_convert_changelogs(changelogs_folder_path);

    // Group the entries by their section
    let grouped = group_by_section(result.clone());

    // Convert the grouped FileEntry's to lines
    let lines = generate_lines(grouped);

    let combined_lines = combine_lines(lines);
    let version_line = create_version_line(new_version, date);

    let old_changelog_content = read_file_to_string(changelog_file_path.clone()).unwrap(); // TODO2: Error handling
    let split_pattern = "\n## ";
    let mut splitted: Vec<&str> = old_changelog_content.splitn(2, split_pattern).collect();

    // Re-Add the pattern that was "splitted" away
    let second_part = splitted.get(1).unwrap();
    let loc = &format!("{}{}", split_pattern, second_part);
    splitted[1] = loc;

    splitted.insert(1, &combined_lines); // Add the actual new lines
    splitted.insert(1, &version_line); // Add version and date

    // Combine all the text again and write it
    let final_text = combine_lines(splitted);
    // TODO2: Proper error handling!
    write_string_to_file(changelog_file_path, final_text).unwrap();

    // 5. Cleanup changelog files
    // TODO: optionally delete the changelog files (based on flag)

    if delete_changelogs {
        let change_logs_file_paths: Vec<String> =
            result.into_iter().map(|fe| fe.path.clone()).collect();
        remove_files(change_logs_file_paths).expect("Couldn't delete all files!");
        // TOOD2: Error handling
    }
}
