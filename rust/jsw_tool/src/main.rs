// use flexi_logger::colored_default_format;
// use log::{debug, error, info, warn};
use std::error::Error;

use clap::Parser;
use cli::{Cli, Commands};

use jsw_binary_lib::add;

mod cli;
mod logging;

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

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name);
    // }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &args.command {
        Commands::Convert(args) => {
            println!("'myapp add' was used, name is: {:?}", args.input);
            add(2, 2);
        }
    }

    Ok(())
}
