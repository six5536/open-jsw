use std::path::PathBuf;

pub fn file_exists(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.is_file() {
        Ok(path)
    } else {
        Err(format!("File not found: {}", s))
    }
}

#[allow(dead_code)]
pub fn dir_exists(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.is_dir() {
        Ok(path)
    } else {
        Err(format!("Dir not found: {}", s))
    }
}
