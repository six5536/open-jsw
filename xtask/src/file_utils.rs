use glob::glob;

/// Recursively find files matching the pattern and apply the action to each file
pub fn for_each_file<F>(pattern: &str, mut action: F)
where
    F: FnMut(&std::path::Path),
{
    for path in glob(pattern)
        .expect("Failed to read glob pattern")
        .flatten()
    {
        action(&path);
    }
}
