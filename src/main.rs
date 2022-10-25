use std::collections::HashMap;
use walkdir::{DirEntry, WalkDir};
// use std::{fs::File, io::BufReader};
use std::error::Error;
use std::fs;

// struct ChangelogSection {
//     title: String,
//     entries: Vec<String>,
// }
#[derive(Clone, Debug)]
struct ChangelogEntry {
    version: String,
    date: String, // TODO: should become a real date!
    sections: HashMap<String, Vec<String>>,
}

#[derive(Clone, Debug)]
struct Changelog {
    title: String,
    description: Vec<String>,
    entries: Vec<ChangelogEntry>,
}

fn convert_changelog_to_string(c: Changelog) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!("# {}", c.title.clone()));

    c.description
        .iter()
        .for_each(|x| lines.push(String::from(x)));

    c.entries.iter().for_each(|entry| {
        lines.push(format!(
            "## [{}] - {}",
            entry.version.clone(),
            entry.date.clone()
        ));

        entry.sections.iter().for_each(|(k, v)| {
            // k -> section name
            lines.push(format!("### {}", k));
            v.iter().for_each(|x| {
                lines.push(x.clone());
            })
        })
    });
    let s = lines.join("\n");
    return s;
}

#[derive(Clone)]
enum CurrentSection {
    TITLE,
    DESC,
    VERSION,
    MOD,
    CHANGE,
}

fn get_title(s: &str) -> &str {
    let (_, title_str) = s.split_at(2); // TODO: maybe rather use an indexof?
    return title_str;
}

// "## [1.0.1] - 2022-06-21" -> ("1.0.1", "022-06-21")
fn parse_version_line(s: &str) -> (String, String) {
    let s1 = s.replace("## ", "");

    // TODO: Handling if there is no " - " !
    let splitted: Vec<&str> = s1.split(" - ").collect();

    // let (version_dirty, date_dirty) =
    let version = splitted
        // TODO: Error handling!
        .get(0)
        .expect("Error by getting the version...")
        .replace(" ", "")
        .replace("[", "")
        .replace("]", "");
    let date_string = splitted
        // TODO: Error handling!
        .get(1)
        .expect("Error by getting the date as string");

    (version, String::from(*date_string))
}

// returns the title
fn handle_title_line(line: &str) -> &str {
    get_title(line)
}

// Only return the description, no specific handling neede
fn handle_desc_line(line: &str) -> &str {
    line
}

// Only return the change, no specific handling neede
fn handle_change_line(line: &str) -> &str {
    line
}

// Returns a new changelog entry
fn handle_version_line(line: &str) -> ChangelogEntry {
    // Format: "## [x.y.z] - YYYY-MM-DD"
    let (version, date_string) = parse_version_line(line);

    // TODO: Parse "date_string" to real date
    ChangelogEntry {
        version: version.clone(),
        date: date_string,
        sections: HashMap::new(),
    }
}

// returns the mod title
fn handle_mod_line(line: &str) -> String {
    let n = line.replace("### ", "");
    return n.clone();
}

fn convert_string_to_changelog(s: String) -> Changelog {
    let lines = s.lines();

    let mut title = String::from("");
    let mut description: Vec<&str> = Vec::new();

    let mut current_section = CurrentSection::TITLE;
    let mut entries: Vec<ChangelogEntry> = Vec::new();
    let mut entries_hash: HashMap<String, ChangelogEntry> = HashMap::new();
    let mut current_version = String::from("");
    let mut current_entry_section = String::from("");

    for line in lines {
        let is_title_line = line.starts_with("# ");
        let is_version_line = line.starts_with("## ");
        let is_mod_line = line.starts_with("### ");

        // first let's change the state based on the special line
        if is_title_line {
            current_section = CurrentSection::TITLE
        }
        if is_version_line {
            current_section = CurrentSection::VERSION
        }
        if is_mod_line {
            current_section = CurrentSection::MOD
        }

        match current_section.clone() {
            CurrentSection::TITLE => {
                title = String::from(get_title(line));
                // Move to next section
                current_section = CurrentSection::DESC;
            }
            CurrentSection::DESC => {
                description.push(line);
            }
            CurrentSection::VERSION => {
                let new_entry = handle_version_line(line);
                let version = new_entry.version.clone();

                entries_hash.insert(version.clone(), new_entry);
                current_version = version.clone();

                // TODO: only logical, since a MOD has to come afters
                // e.g. "### Fixed"
                current_section = CurrentSection::MOD;
            }
            CurrentSection::MOD => {
                // e.g. "### Fixed"
                let section_name = handle_mod_line(line);
                let v = entries_hash.get_mut(&current_version).unwrap();

                // FIXME: duplicated seciton names!!
                v.sections
                    .insert(String::from(section_name.clone()), Vec::new());
                current_entry_section = section_name;
                // TODO: check if it works or if we have to insert again
                // In any case, don#t use mutable but get it, modify it and then set again (functional!)

                // entries_hash.insert(String::from(current_version), *val);
                current_section = CurrentSection::CHANGE;
            }
            CurrentSection::CHANGE => {
                let v = entries_hash.get_mut(&current_version).unwrap();
                let sec = v.sections.get_mut(&current_entry_section).unwrap();
                // FIXME: check if it's enough or if we need to insert again
                sec.push(String::from(line.clone()));
            }
        }
        continue;
    }

    // FIXME: Implement
    Changelog {
        title: title.clone(),
        description: description.iter().map(|x| String::from(*x)).collect(),
        // FIXME: order the "entries!"
        entries: entries_hash.iter().map(|(_, v)| v.clone()).collect(),
    }
}

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

    // TODO: Those should be passed in via a config file later!
    let changelogs_folder_path = String::from("examples/demo1/changelogs");
    let changelog_file = String::from("examples/demo1/CHANGELOG.md");

    // 1. Find the changelog file

    // 2. Find the changelogs to add
    let file_paths = find_changelogs(changelogs_folder_path);

    println!("{:?}", file_paths);

    // 3. Parse the changelogs (create the entries)
    let changelog = read_file(changelog_file.clone()).expect("seems like this program crashed..:/");

    // println!("\n");
    // println!("\n");
    // println!("====== Changelog Debug ======");
    // println!("{:?}", changelog);

    let stringified = convert_changelog_to_string(changelog);
    // println!("\n");
    // println!("\n");
    // println!("====== Stringified Debug ======");
    // println!("{}", stringified);

    // 4. Add the changelogs

    // 5. Cleanup changelog files

    // 6. Set version + Date
}
