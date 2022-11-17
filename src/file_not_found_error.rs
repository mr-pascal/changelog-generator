use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FileNotFoundError {
    file_path: String,
}

impl fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ERROR: Could not find or read the file '{}'... Are you sure you provided the correct path?",
            self.file_path
        )
    }
}

impl Error for FileNotFoundError {}

impl From<String> for FileNotFoundError {
    fn from(file_path: String) -> Self {
        FileNotFoundError { file_path }
    }
}

impl FileNotFoundError {
    pub fn new(file_path: String) -> FileNotFoundError {
        FileNotFoundError { file_path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() -> Result<(), String> {
        let path = "./my/file/path.md".to_owned();

        let output = FileNotFoundError::new(path.clone());
        let expected = FileNotFoundError {
            file_path: path.clone(),
        };

        assert_eq!(output.file_path, expected.file_path);

        Ok(())
    }

    #[test]
    fn test_fmt() -> Result<(), String> {
        let path = "./my/file/path.md".to_owned();
        let e = FileNotFoundError::new(path.clone());

        let output = format!("{}", e);
        let expected = format!("ERROR: Could not find or read the file '{}'... Are you sure you provided the correct path?", path.clone());

        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn test_from() -> Result<(), String> {
        let path = "./my/file/path.md".to_owned();
        let e = FileNotFoundError::from(path.clone());

        let expected = path.clone();
        let output = e.file_path;

        assert_eq!(output, expected);
        Ok(())
    }
}
