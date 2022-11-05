mod file_entry;
mod filesystem;
mod parser;
mod walker;
use clap::Parser;
use file_entry::{group_by_section, FileEntry};
use filesystem::{read_file_to_string, write_string_to_file};
use parser::{create_version_line, parse_file_name};
use walker::find_changelogs;

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
