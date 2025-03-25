// use clap::value_parser;

use clap::{Args, Parser, Subcommand, ValueEnum};

pub mod validation;

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
    /// Print information about the project including the targets
    Info(InfoArgs),

    /// Build an application
    Build(BuildArgs),

    /// Run an application
    Run(RunArgs),

    /// Build an application for release
    Dist(DistArgs),

    /// Convert ./resources/mm/bin/mm.tzx to ./resources/mm/game
    ConvertMm,

    /// Convert ./resources/jsw/bin/jsw.tzx to ./resources/jsw/game
    ConvertJsw,

    /// Convert ./resources/jsw2/bin/jsw2.tzx to ./resources/jsw2/game
    ConvertJsw2,
}

#[derive(Args)]
pub struct InfoArgs {
    // /// Path to original binary game
    // #[arg(value_parser = file_exists)]
    // pub input: PathBuf,

    // /// Output directory
    // // #[arg(value_parser = dir_exists)]
    // pub output: Option<PathBuf>,
    // // output: Option<String>,
}

#[derive(Args)]
pub struct BuildArgs {
    /// Build target
    #[arg(value_parser = validation::valid_build_target)]
    pub target: Option<String>,
    // /// Path to original binary game
    // #[arg(value_parser = file_exists)]
    // pub input: PathBuf,

    // /// Output directory
    // // #[arg(value_parser = dir_exists)]
    // pub output: Option<PathBuf>,
    // // output: Option<String>,
}

#[derive(Args)]
pub struct RunArgs {
    /// Run target
    #[arg(value_parser = validation::valid_run_target)]
    pub target: Option<String>,
    // /// Path to original binary game
    // #[arg(value_parser = file_exists)]
    // pub input: PathBuf,

    // /// Output directory
    // // #[arg(value_parser = dir_exists)]
    // pub output: Option<PathBuf>,
    // // output: Option<String>,
}

#[derive(Args)]
pub struct DistArgs {
    /// Run target
    #[arg(value_parser = validation::valid_run_target)]
    pub target: Option<String>,
    // /// Path to original binary game
    // #[arg(value_parser = file_exists)]
    // pub input: PathBuf,

    // /// Output directory
    // // #[arg(value_parser = dir_exists)]
    // pub output: Option<PathBuf>,
    // // output: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum BuildTarget {
    All, // All targets
    JswTool,
    OpenJsw,
    OpenJswCore,
    OpenJswTiled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum RunTarget {
    All, // All targets
    JswTool,
    OpenJsw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DistTarget {
    All, // All targets
    JswTool,
    OpenJsw,
}
