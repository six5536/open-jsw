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

pub fn is_not_file_and_parent_dir_exists(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);

    if path.is_file() {
        return Err(format!("Path is file: {}", s));
    }

    let parent_path = path
        .parent()
        .ok_or(format!("Parent dir not found for path: {}", s))?;

    if parent_path.is_dir() {
        Ok(path)
    } else {
        Err(format!("Parent dir not found for path: {}", s))
    }
}

pub fn is_not_dir_and_parent_dir_exists(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);

    if path.is_dir() {
        return Err(format!("Path is directory: {}", s));
    }

    let parent_path = path
        .parent()
        .ok_or(format!("Parent dir not found for path: {}", s))?;

    if parent_path.is_dir() {
        Ok(path)
    } else {
        Err(format!("Parent dir not found for path: {}", s))
    }
}
