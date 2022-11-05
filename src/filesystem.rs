use std::error::Error;
use std::fs;

pub fn read_file_to_string(path: String) -> Result<String, Box<dyn Error>> {
    let out: String = fs::read_to_string(&path)?.parse()?;
    Ok(out)
}

pub fn write_string_to_file(path: String, content: String) -> Result<(), Box<dyn Error>> {
    fs::write(path, content)?;
    Ok(())
}
