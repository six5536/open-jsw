// use derive_more::From;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;
// pub type Error = Box<dyn std::error::Error>; // For early dev

#[derive(Error, Debug)]
pub enum Error {
    // Custom errors (for early dev)
    #[error("{}", .0)]
    Custom(String),

    // Module errors
    #[error("Index {} is out of bounds 0..{}", .index, .length)]
    IndexOutOfBounds { index: usize, length: usize },

    #[error("Game not recognised")]
    GameNotRecognised,

    #[error("Game room conversion failed [{}]: {:?}", .message, .mode)]
    GameConversionFailed {
        mode: GameConversionError,
        message: String,
    },

    // External errors
    #[error("IO::{:?}: {}", .0, .0)]
    Io(#[from] std::io::Error),
    // #[error("FlexiLogger::{:?}: {}", .0, .0)]
    // FlexiLogger(#[from] flexi_logger::FlexiLoggerError),
}

#[derive(Debug)]
pub enum GameConversionError {
    RoomConversionFailed { room: String },
}

// impl Error {
//     //
// }

// region:    --- Error Boilerplate

// impl core::fmt::Display for Error {
//     fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
//         write!(fmt, "{self:?}")
//     }
// }

// impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
