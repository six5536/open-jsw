pub mod formats;

use flexi_logger::Logger;
use std::error::Error;

pub fn init() -> Result<(), Box<dyn Error>> {
    Logger::try_with_env_or_str("info")?
        .format(formats::cli_format)
        .log_to_stdout()
        .start()?;

    Ok(())
}
