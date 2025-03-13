// Import and re-export the `error` module
pub use self::error::{Error, Result};
mod error;

mod converter;

use converter::jsw_raw::JswRaw;
use std::io::Read;

pub fn convert(rdr: impl Read) -> Result<JswRaw> {
    converter::jsw_raw::from_reader(rdr)
    // Err(io::Error::new(io::ErrorKind::Other, "your message here"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
