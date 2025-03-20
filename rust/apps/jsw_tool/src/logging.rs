pub mod formats;

use flexi_logger::Logger;

use crate::Error;

pub fn init() -> Result<(), Error> {
    Logger::try_with_env_or_str("info")?
        .format(formats::cli_format)
        .log_to_stdout()
        .start()?;

    Ok(())
    // Err(FlexiLoggerError::NoDuplication.into())
    // // Err(Error::TestError {
    // //     message: "Test error".to_string(),
    // //     num: 42,
    // // })
}
