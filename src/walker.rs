use crate::filesystem::read_file_to_string;

use super::parser::parse_file_name;
use super::FileEntry;
use walkdir::WalkDir;

pub fn find_and_convert_changelogs(folder_path: String) -> Vec<FileEntry> {
    // Format -> <ticket_number>_<action>_<random>.md"

    let mut file_entries: Vec<FileEntry> = vec![];

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

            // TODO: Put conversion logic to dedicated method!

            // TODO2: Error Handling
            let path = x.path().to_str().unwrap().to_owned();
            // TODO2: Error Handling
            let file_name = x.file_name().to_str().unwrap().to_owned();

            // Read file
            //TODO2: Error handling
            let content = read_file_to_string(path.clone()).unwrap();

            match parse_file_name(file_name.clone()) {
                Ok((ticket_reference, section)) => {
                    // Create full entry
                    file_entries.push(FileEntry {
                        file_name,
                        path,
                        content,
                        ticket_reference,
                        section,
                    });
                }
                Err(e) => println!("{}", e),
            }
        });
    file_entries
}
