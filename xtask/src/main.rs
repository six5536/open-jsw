use std::{
    fs,
    process::{Command, Stdio},
};

pub use self::error::{Error, Result};
mod error;

use crate::project::{cargo_path, dist_dir, project_root};
use clap::Parser;
use cli::{Cli, Commands};

mod cli;
mod commands;
mod file_utils;
mod project;

// Libraries:
// devx: collection of useful utilities (spawning processes, git pre-commit hooks, etc.)
// xshell: ergonomic "bash" scripting in Rust
// duct: a library for running child processes with support for pipelines and IO redirection

fn main() -> Result<()> {
    if let Err(e) = run() {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
    Ok(())
}

fn run() -> Result<()> {
    let args = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &args.command {
        Commands::Info(_args) => commands::info()?,
        Commands::Build(args) => commands::build(args)?,
        Commands::Run(args) => commands::run(args)?,
        Commands::Dist(args) => commands::dist(args)?,
        Commands::ConvertMm => commands::convert_mm()?,
    }

    Ok(())
}

fn dist() -> Result<()> {
    let _ = fs::remove_dir_all(dist_dir());
    fs::create_dir_all(dist_dir())?;

    dist_binary()?;
    // dist_manpage()?;

    Ok(())
}

fn dist_binary() -> Result<()> {
    let status = Command::new(cargo_path())
        .current_dir(project_root())
        .args(["build", "--release"])
        .status()?;

    if !status.success() {
        Err(Error::Text("cargo build failed".into()))?;
    }

    let dst = project_root().join("target/release/hello-world");

    fs::copy(&dst, dist_dir().join("hello-world"))?;

    if Command::new("strip")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
    {
        eprintln!("stripping the binary");
        let status = Command::new("strip").arg(&dst).status()?;
        if !status.success() {
            Err(Error::Text("strip failed".into()))?;
        }
    } else {
        eprintln!("no `strip` utility found")
    }

    Ok(())
}
