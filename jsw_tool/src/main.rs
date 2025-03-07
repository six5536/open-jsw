use anyhow::{Context, Result};
use clap::Parser;

// use flexi_logger::colored_default_format;
// use log::{debug, error, info, warn};
use std::error::Error;

mod logging;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
    // pattern: String,
    // path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    // println!("{} {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    logging::init()?;

    // env_logger::init();
    // info!("JSW Tool");
    // debug!("JSW Tool");
    // warn!("JSW Tool");
    // error!("JSW Tool");

    let args = Cli::parse();
    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    // let content = std::fs::read_to_string(&args.path)
    //     .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    Ok(())
}
