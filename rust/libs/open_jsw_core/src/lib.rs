// use raw_game::JswRawGame;

// Import and re-export the `error` module
pub use self::error::{Error, Result};
mod error;

pub mod converter;
pub mod game;
mod image;
pub mod raw_game;
mod zx;

// use std::io::Read;

// pub fn convert(rdr: impl Read) -> Result<JswRawGame> {
//     raw_game::JswRawGame::from_reader(rdr)
//     // Err(io::Error::new(io::ErrorKind::Other, "your message here"))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
