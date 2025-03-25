use std::path::PathBuf;

use crate::project::{Package, get_packages};

pub fn valid_build_target(s: &str) -> Result<String, String> {
    let packages = get_packages();
    let package = packages.iter().find(|p| p.name == s);
    match package {
        Some(p) => Ok(p.name.clone()),
        None => {
            println!();
            print_build_targets(&packages);
            println!();
            Err(format!("Package not found: {}", s))
        }
    }
}

pub fn valid_run_target(s: &str) -> Result<String, String> {
    let packages = get_packages();
    let package = packages.iter().find(|p| p.name == s);
    match package {
        Some(p) => Ok(p.name.clone()),
        None => {
            println!();
            print_run_targets(&packages);
            println!();
            Err(format!("Binary package not found: {}", s))
        }
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

pub fn print_build_targets(packages: &Vec<Package>) {
    println!("# Available targets");
    println!();
    println!("[bin]");
    print_binary_targets(packages);
    println!();
    println!("[lib]");
    print_library_targets(packages);
}

pub fn print_run_targets(packages: &Vec<Package>) {
    println!("# Available targets");
    println!();
    println!("[bin]");
    print_binary_targets(packages);
}

pub fn print_binary_targets(packages: &Vec<Package>) {
    for p in packages {
        if p.is_bin {
            println!("{}", p.name);
        }
    }
}

pub fn print_library_targets(packages: &Vec<Package>) {
    for p in packages {
        if p.is_lib {
            println!("{}", p.name);
        }
    }
}
