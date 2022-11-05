// use super::FileEntry;
use std::error::Error;
use std::fs;
// use walkdir::WalkDir;

pub fn read_file_to_string(path: String) -> Result<String, Box<dyn Error>> {
    let out: String = fs::read_to_string(&path)?.parse()?;
    Ok(out)
}

pub fn write_string_to_file(path: String, content: String) -> Result<(), Box<dyn Error>> {
    fs::write(path, content)?;
    Ok(())
}

// pub fn find_changelogs(folder_path: String) -> Vec<FileEntry> {
//     // Format -> <ticket_number>_<action>_<random>.md"

//     let mut file_entries: Vec<FileEntry> = vec![];
//     // println!("find_changelogs: searching in '{}'", folder_path);

//     WalkDir::new(folder_path)
//         .into_iter()
//         // .filter_entry(|e| !is_hidden(e))
//         .filter_map(|v| v.ok())
//         .for_each(|x| {
//             if x.depth() != 1 {
//                 // Ignore everything which isn't a direct
//                 // descendant of 'folder_path'
//                 return;
//             }

//             file_entries.push(FileEntry {
//                 file_name: x.file_name().to_str().unwrap().to_owned(), // TODO2: Maybe some error handling later
//                 path: x.path().to_str().unwrap().to_owned(), // TODO2: maybe some error handling later
//                 content: String::from(""),
//                 ticket_reference: String::from(""),
//                 section: String::from(""),
//             });
//         });
//     return file_entries;
// }
