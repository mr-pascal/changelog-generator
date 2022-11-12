use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub file_name: String,
    pub path: String,
    pub content: String,
    pub ticket_reference: String,
    pub section: String,
}

pub fn group_by_section(entries: Vec<FileEntry>) -> HashMap<String, Vec<FileEntry>> {
    let mut hm: HashMap<String, Vec<FileEntry>> = HashMap::new();
    entries.into_iter().for_each(|entry| {
        let section: String = entry.section.clone();
        if !hm.contains_key(&section) {
            // nothing in there yet
            hm.insert(section.clone(), vec![]);
        }
        // get mut so we don't have to re-insert it
        let v = hm.get_mut(&section);
        match v {
            Some(v) => v.push(entry),
            // Don't do any special handling here, just ignore and print
            None => eprintln!("WARN: Haven't found section '{}'", &section),
        }
    });
    hm
}

#[cfg(test)]
mod tests {
    use super::*;

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
