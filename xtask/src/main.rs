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
        Commands::ConvertJsw => commands::convert_jsw()?,
        Commands::ConvertJsw2 => commands::convert_jsw2()?,
    }

    Ok(())
}
