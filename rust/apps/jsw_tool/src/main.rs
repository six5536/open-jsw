use std::fs::{self};

// Import and re-export the `error` module
pub use self::error::{Error, Result};
mod error;

use clap::Parser;
use cli::{Cli, Commands};
use open_jsw_core::{
    converter::{Converter, raw_to_tiled_converter::RawToTiledConverter},
    raw_game::JswRawGame,
};

mod cli;
mod logging;

fn main() -> Result<()> {
    if let Err(e) = run() {
        log::error!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}

fn run() -> Result<()> {
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
            println!("Converting: {:?}", args.input);

            let input_path = &args.input;
            // let file = File::open(path)
            //     .with_context(|| format!("Failed to load conversion input '{:?}'", path))?;
            // let res = convert(file).with_context(|| format!("Failed to convert '{:?}'", path))?;

            let raw_game = JswRawGame::from_file(input_path)?;
            for room in &raw_game.rooms {
                println!("{} - {:?}", room.room_no, room.name);
            }
            // println!("{:?}", raw_game.rooms);

            let converter = RawToTiledConverter;

            let game = converter.convert(&raw_game);
            // println!("{:?}", game);

            let json = open_jsw_tiled::serialize_map(&game)?;

            // Write the converted game to a file
            if let Some(output_path) = &args.output {
                fs::write(output_path.as_path(), &json)?;
            }

            // fs::write(output_path.as_path(), &json)?;
        }
        Commands::ReadMap(args) => {
            println!("Reading Tiled map: {:?}", args.input);

            let path = &args.input;
            // let file = File::open(path)
            //     .with_context(|| format!("Failed to load conversion input '{:?}'", path))?;
            // let res = convert(file).with_context(|| format!("Failed to convert '{:?}'", path))?;

            // let file = File::open(path)?;
            let data = fs::read_to_string(path)?;

            let res = open_jsw_tiled::deserialize_map(&data)?;
            // for room in res.rooms {
            //     println!("{} - {:?}", room.room_no, room.name);
            // }
            println!("{:?}", res);
        }
    }

    Ok(())
}
