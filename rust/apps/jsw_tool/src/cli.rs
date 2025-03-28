// use clap::value_parser;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

mod validation;
use validation::{file_exists, is_not_file_and_parent_dir_exists};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    // /// Name of the person to greet
    // #[arg(short, long)]
    // name: String,

    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
    #[command(subcommand)]
    pub command: Commands,
    // pattern: String,
    // path: std::path::PathBuf,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert an original game file to OpenJSW format
    Convert(ConvertArgs),

    ///  Read a Tiled map
    ReadMap(ConvertArgs),
}

#[derive(Args)]
pub struct ConvertArgs {
    /// Path to original binary game
    #[arg(value_parser = file_exists)]
    pub input: PathBuf,

    /// Output directory
    #[arg(value_parser = is_not_file_and_parent_dir_exists)]
    pub output: PathBuf,
    // output: Option<String>,
}

#[derive(Args)]
pub struct ReadMapArgs {
    /// Path to original binary game
    #[arg(value_parser = file_exists)]
    pub input: PathBuf,

    /// Output directory
    // #[arg(value_parser = dir_exists)]
    pub output: Option<PathBuf>,
    // output: Option<String>,
}
