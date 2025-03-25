use std::{
    env, fs,
    path::{Path, PathBuf},
};

use toml::Value;

use crate::file_utils::for_each_file;

pub struct Package {
    pub name: String,
    pub is_bin: bool,
    pub is_lib: bool,
}

/// Get the project root directory
pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

/// Get the distribution directory
pub fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}

/// Get the release directory
pub fn release_dir() -> PathBuf {
    project_root().join("target/release")
}

/// Get the path to the `cargo` executable
pub fn cargo_path() -> String {
    env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

/// Parse the cargo.toml manifest files to extract the package names and types
pub fn get_packages() -> Vec<Package> {
    let mut packages = Vec::new();

    // Recursively find Cargo.toml files
    let project_root = project_root();
    let path_pattern = project_root.join("**/Cargo.toml");
    let pattern: &str = &path_pattern.to_string_lossy();

    for_each_file(pattern, |path| {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(toml) = contents.parse::<Value>() {
                if let Some(package_name) = toml.get("package").and_then(|p| p.get("name")) {
                    // Skip xtask itself
                    if package_name.as_str() == Some("xtask") {
                        return;
                    }

                    if let Some(package_name) = package_name.as_str() {
                        let has_lib = toml.get("lib").is_some();
                        let has_bin = toml
                            .get("bin")
                            .is_some_and(|b| b.as_array().is_some_and(|a| !a.is_empty()));

                        // If Cargo.toml doesn't explicitly define bin/lib, check file system defaults
                        let project_root = path.parent().unwrap_or(Path::new("."));
                        let default_lib = project_root.join("src/lib.rs").exists();
                        let default_bin = project_root.join("src/main.rs").exists();

                        let has_lib = has_lib || default_lib;
                        let has_bin = has_bin || default_bin;

                        packages.push(Package {
                            name: package_name.into(),
                            is_bin: has_bin,
                            is_lib: has_lib,
                        });
                    }
                }
            }
        }
    });

    packages
}
