use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ChangelogEntry {
    pub version: String,
    pub date: String, // TODO: should become a real date!
    pub sections: HashMap<String, Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct Changelog {
    pub title: String,
    pub description: Vec<String>,
    pub entries: Vec<ChangelogEntry>,
}
