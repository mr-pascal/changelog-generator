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

pub fn remove_files(paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    paths.into_iter().for_each(|path| {
        // TODO2: don't break but delete all files possible
        let err_msg: &str = &format!("Couldn't delete file '{}'!", path);
        fs::remove_file(path).expect(err_msg); // TODO2: Error hadnling
    });
    Ok(())
}
